use super::response::{Data, Response};
use actix_web::{error::Error as ActixError, HttpRequest, HttpResponse};
use serde::Serialize;

pub trait IntoRes<T> {
    fn into_res(self, req: &mut HttpRequest) -> Result<HttpResponse, ActixError>;
}

impl<T: Serialize> IntoRes<T> for Result<Data<T>, Response> {
    fn into_res(self, req: &mut HttpRequest) -> Result<HttpResponse, ActixError> {
        match self {
            Ok(v) => Ok(v.into_req(req)),
            Err(e) => Err(e.into_req(req)),
        }
    }
}

/*
    handler(pool, p1, p2) -> Result<HttpResponse, ActixError> {
        db_model::call(pol, p1, p2).into_req()
    }

    call(pool, p1, p2) -> Result<Data<T>, Response> {
        match func() {
            Ok(v) => Ok(v.into),
            Err(e) => Err(Response::not_found("item".into())),
        }
    }
*/
