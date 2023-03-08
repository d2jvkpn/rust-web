// https://actix.rs/docs/middleware/
use super::response;
use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::{HeaderName, HeaderValue},
    HttpMessage, HttpRequest,
};
use chrono::{DateTime, Local};
use futures_util::future::LocalBoxFuture;
use log::{error, info, warn};
use serde::Serialize;
use serde_json::json;
use std::future::{ready, Ready};
use uuid::Uuid;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Logger {
    pub get_user_id: fn(&HttpRequest) -> Option<i32>,
}

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
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
        ready(Ok(LoggerMiddleware { service, get_user_id: self.get_user_id }))
    }
}

pub struct LoggerMiddleware<S> {
    service: S,
    get_user_id: fn(&HttpRequest) -> Option<i32>,
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

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        //
        let start: DateTime<Local> = Local::now();
        let request_id = Uuid::new_v4();

        let mut record = Record {
            method: req.method().to_string(),
            path: req.path().to_string(),
            request_id,
            user_id: (self.get_user_id)(req.request()),
            ..Default::default()
        };

        req.extensions_mut().insert(request_id);

        req.headers_mut().insert(
            HeaderName::from_lowercase(b"x-request-id").unwrap(),
            HeaderValue::from_str(request_id.to_string().as_str()).unwrap(),
        );

        let fut = self.service.call(req);

        Box::pin(async move {
            let result = fut.await;

            let end: DateTime<Local> = Local::now();
            let elapsed = end.signed_duration_since(start).num_microseconds().unwrap_or(0);
            record.elapsed = format!("{:.3}ms", (elapsed as f64) / 1e3);

            // Result<ServiceResponse<B>, actix_web::Error>
            // v.response(): &HttpResponse<B>, e.error_response(): HttpResponse
            let call = |headers, mut record: Record| {
                if let Some(v) = response::Error::extract_from_headers(headers) {
                    (record.code, record.msg) = (Some(v.0), Some(v.1));
                }
                record.log();
            };

            match result {
                Ok(mut v) => {
                    record.status = v.status().as_u16();
                    call(v.headers_mut(), record);
                    Ok(v)
                }
                Err(e) => {
                    let mut res = e.error_response();
                    record.status = res.status().as_u16();
                    call(res.headers_mut(), record);
                    Err(e)
                }
            }
            // exts = HttpResponse.extensions(); data = exts.get::<JwtPayload>()?; data.user_id
        })
    }
}

#[derive(Debug, Serialize, Default)]
struct Record {
    pub request_id: Uuid,
    pub method: String,
    pub path: String,
    pub status: u16,
    pub user_id: Option<i32>,
    pub elapsed: String,
    pub code: Option<i32>,
    pub msg: Option<String>,
}

impl Record {
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
