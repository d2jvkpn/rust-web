// https://actix.rs/docs/middleware/
use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    HttpMessage, HttpRequest,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

pub struct Blocker<T> {
    pub block: fn(&HttpRequest) -> Result<T, actix_web::Error>,
}

impl<S, B, T: 'static> Transform<S, ServiceRequest> for Blocker<T>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = BlockerMiddleware<S, T>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(BlockerMiddleware { block: self.block, service }))
    }
}

#[allow(dead_code)]
pub struct BlockerMiddleware<S, T> {
    block: fn(&HttpRequest) -> Result<T, actix_web::Error>,
    service: S,
}

impl<S, B, T: 'static> Service<ServiceRequest> for BlockerMiddleware<S, T>
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
        let item = match (self.block)(req.request()) {
            Ok(v) => v,
            Err(e) => return Box::pin(ready(Err(e))),
        };
        req.extensions_mut().insert(item);

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
