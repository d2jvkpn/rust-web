// https://actix.rs/docs/middleware/
use super::{record::Record, Res};
use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    http::Method,
    HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

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
        let is_options = req.method() == Method::OPTIONS;
        let mut record = Record::from_request(req.request());
        req.extensions_mut().insert(record.request_id);

        let fut = self.service.call(req);

        Box::pin(async move {
            let result = fut.await;
            record.elapsed();

            let sr = match result {
                Ok(v) => v,
                Err(e) => {
                    // dbg!(&e);
                    if !is_options {
                        record.cause = Some(format!("{:}", e));
                        let mut res = e.error_response();
                        handle_unexpectd(&mut record, &mut res);
                    }

                    return Err(e);
                }
            };

            let req = sr.request().clone();
            let mut exts = req.extensions_mut();
            record.user_id = exts.get::<i32>().copied();

            if let Some(trace) = exts.remove::<Res>() {
                match trace {
                    Res::Ok => {
                        record.status = sr.response().status().as_u16();
                        record.msg = "ok".into();
                    }
                    Res::Err(e) => record.with_error(e),
                }
            } else {
                record.code = -1001;
                record.status = sr.response().status().as_u16();
                record.msg = "HAS NO TRACE".into();
            }

            record.log();
            Ok(sr)
        })
    }
}

fn handle_unexpectd(record: &mut Record, res: &mut HttpResponse) {
    record.msg = "UNEXPECTED ERROR".into();
    record.code = -1000;
    record.status = res.status().as_u16();
    // record.cause = Some(format!("{:}", e));
    let exts = res.extensions_mut();
    record.user_id = exts.get::<i32>().copied();

    record.log();
}
