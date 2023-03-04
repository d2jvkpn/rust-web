// https://actix.rs/docs/middleware/
use crate::{internal::settings::Config, middlewares::response};
use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::AUTHORIZATION,
    HttpMessage,
};
use chrono::Utc;
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

pub struct Auth;

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
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
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
        let err = response::Error::Unauthenticated(
            "a1::you are not logged in, please provide token".to_string(),
        );
        let value = match req.headers().get(AUTHORIZATION) {
            Some(v) => v,
            None => return Box::pin(ready(Err(err.into()))),
        };

        let err = response::Error::Unauthenticated("a2::invalid token".to_string());
        let token = match value.to_str() {
            Ok(v) => v,
            Err(_) => return Box::pin(ready(Err(err.into()))),
        };

        let payload = match Config::jwt_verify(token.to_string()) {
            Ok(v) => v,
            Err(e) => return Box::pin(ready(Err(e.into()))),
        };

        let err = response::Error::Unauthenticated("a3::token expired".into());
        if payload.iat > Utc::now().timestamp() {
            return Box::pin(ready(Err(err.into())));
        }

        req.extensions_mut().insert(payload);

        //
        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
