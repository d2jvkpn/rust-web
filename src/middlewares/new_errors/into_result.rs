// use super::{data_v1::Data, error::Error};
use super::{data::Data, errors::Error};
use actix_web::{error::Error as ActixError, HttpRequest, HttpResponse};
use serde::Serialize;

pub trait IntoRessult<T> {
    fn into_res(self, req: &mut HttpRequest) -> Result<HttpResponse, ActixError>;
}

// impl<T: Serialize> IntoRes<T> for Result<Data<T>, Error> {
impl<T: Serialize> IntoRessult<T> for Result<T, Error> {
    fn into_res(self, req: &mut HttpRequest) -> Result<HttpResponse, ActixError> {
        match self {
            Ok(v) => {
                let d = Data(v);
                Ok(d.into_req(req))
            }
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
