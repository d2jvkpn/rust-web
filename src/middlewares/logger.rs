// https://actix.rs/docs/middleware/
use super::errors::Error as AnError;
use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    HttpMessage, HttpRequest,
};
use chrono::{DateTime, Local};
use futures_util::future::LocalBoxFuture;
use log::{error, info, warn};
use serde::Serialize;
use serde_json::json;
use std::future::{ready, Ready};
use uuid::Uuid;

pub struct Logger {
    // pub get_user_id: fn(&HttpRequest) -> Option<i32>,
}

impl<S, B> Transform<S, ServiceRequest> for Logger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = LoggerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        // ready(Ok(LoggerMiddleware { service, get_user_id: self.get_user_id }))
        ready(Ok(LoggerMiddleware { service }))
    }
}

pub struct LoggerMiddleware<S> {
    service: S,
    // get_user_id: fn(&HttpRequest) -> Option<i32>,
}

impl<S, B> Service<ServiceRequest> for LoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let mut record = Record::from_request(req.request());

        let fut = self.service.call(req);

        Box::pin(async move {
            let result = fut.await;
            record.elapsed();

            match result {
                Ok(v) => {
                    let req = v.request().clone();
                    let mut exts = req.extensions_mut();
                    record.user_id = exts.remove::<String>();

                    if let Some(err) = exts.remove::<AnError>() {
                        record.with_error(err);
                    } else {
                        record.status = v.response().status().as_u16();
                        record.msg = Some("ok".into());
                        record.request_id = exts.remove::<Uuid>().unwrap();
                    }
                    record.log();
                    Ok(v)
                }
                Err(e) => {
                    let mut res = e.error_response();
                    record.status = res.status().as_u16();
                    record.cause = Some(format!("{:}", e));
                    let mut exts = res.extensions_mut();
                    record.user_id = exts.remove::<String>();
                    // TODO:...

                    record.log();
                    Err(e)
                }
            }
        })
    }
}

#[derive(Debug, Serialize, Default)]
struct Record {
    pub start_at: DateTime<Local>,

    pub request_id: Uuid,
    pub method: String,
    pub path: String,
    pub status: u16,
    pub user_id: Option<String>,
    pub elapsed: String,
    pub code: i32,
    pub msg: Option<String>,
    pub cause: Option<String>,
    pub loc: Option<String>,
}

impl Record {
    pub fn from_request(req: &HttpRequest) -> Self {
        Record {
            start_at: Local::now(),
            method: req.method().to_string(),
            path: req.path().to_string(),
            request_id: Uuid::nil(),
            ..Default::default()
        }
    }

    pub fn elapsed(&mut self) {
        let end: DateTime<Local> = Local::now();
        let elapsed = end.signed_duration_since(self.start_at).num_microseconds().unwrap_or(0);
        self.elapsed = format!("{:.3}ms", (elapsed as f64) / 1e3);
    }

    // consume a response::Response, using Option<T>.take() rather than Option<T>.clone()
    pub fn with_error(&mut self, mut err: AnError) {
        self.request_id = err.request_id;
        self.code = err.code;
        self.msg = err.msg.take();
        self.status = err.status.as_u16();
        if let Some(e) = err.cause {
            self.cause = Some(format!("{:}", e));
        }
        self.loc = err.loc.take();
    }

    pub fn log(&self) {
        if self.status >= 500 {
            error!("{}", json!(self));
        } else if self.status >= 400 {
            warn!("{}", json!(self));
        } else {
            info!("{}", json!(self));
        }
    }
}
