// https://actix.rs/docs/middleware/
use crate::{internal::settings::Settings, middlewares::Error, models::user::Role};
use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

pub struct Auth {
    pub value: Role,
}

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { value: self.value.clone(), service }))
    }
}

#[allow(dead_code)]
pub struct AuthMiddleware<S> {
    value: Role,
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
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

        let payload = match Settings::jwt_verify_request(req.request()) {
            Ok(v) => v,
            Err(e) => return Box::pin(ready(Err(e.into()))),
        };

        if payload.role != self.value {
            let err = Error::permission_denied().msg("you have no permission");
            return Box::pin(ready(Err(err.into())));
        }

        req.extensions_mut().insert(payload);

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
