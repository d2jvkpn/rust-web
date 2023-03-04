// https://actix.rs/docs/middleware/
use super::response;
use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    HttpMessage, HttpRequest,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

pub struct Auth<T> {
    pub verify: fn(&HttpRequest) -> Result<T, response::Error>,
}

impl<S, B, T: 'static> Transform<S, ServiceRequest> for Auth<T>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = AuthMiddleware<S, T>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { verify: self.verify, service }))
    }
}

#[allow(dead_code)]
pub struct AuthMiddleware<S, T> {
    verify: fn(&HttpRequest) -> Result<T, response::Error>,
    service: S,
}

impl<S, B, T: 'static> Service<ServiceRequest> for AuthMiddleware<S, T>
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
        // println!("~~~ {}", self.value);

        let item = match (self.verify)(req.request()) {
            Ok(v) => v,
            Err(e) => return Box::pin(ready(Err(e.into()))),
        };
        req.extensions_mut().insert(item);

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
