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
        // !! insertion codes here: Handle_ServiceRequest_Before
        let item = match (self.block)(req.request()) {
            Ok(v) => v,
            Err(e) => return Box::pin(ready(Err(e))),
        };
        req.extensions_mut().insert(item);

        let fut = self.service.call(req);
        Box::pin(async move {
            let result = fut.await;
            // !! insertion codes here: Handle_ServiceRequest_After, Handle_Error
            // result: Result<ServiceRequest<B>, actix_web::Error>
            // https://docs.rs/actix-web/latest/actix_web/dev/struct.ServiceResponse.html
            // https://docs.rs/actix-web/4.3.1/actix_web/error/struct.Error.html

            // HttpResponse.extensions().get::<T>()?
            result
        })
    }
}
