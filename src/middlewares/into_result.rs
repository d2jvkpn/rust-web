// use super::{data_v1::Data, error::Error};
use super::errors::Error;
use actix_web::{error::Error as ActixError, HttpMessage, HttpRequest, HttpResponse};
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

pub const OK_JSON: &str = r#"{"code":0,"msg":"ok","data":{}}"#;

#[derive(Debug, Serialize)]
pub struct Data<T>(pub T);

pub enum Trace {
    RequestId(Uuid),
    Error(Error),
}

pub fn empty_data() -> HashMap<u8, u8> {
    HashMap::new()
}

impl<T: Serialize> From<T> for Data<T> {
    fn from(d: T) -> Self {
        Data(d)
    }
}

pub trait IntoResult<T> {
    fn into_result(self, req: &mut HttpRequest) -> Result<HttpResponse, ActixError>;
}

// impl<T: Serialize> IntoRes<T> for Result<Data<T>, Error> {
impl<T: Serialize> IntoResult<T> for Data<T> {
    fn into_result(self, req: &mut HttpRequest) -> Result<HttpResponse, ActixError> {
        let request_id = Uuid::new_v4();
        req.extensions_mut().insert(Trace::RequestId(request_id));

        Ok(HttpResponse::Ok()
            .json(json!({"code": 0, "msg":"ok", "requestId": request_id, "data": self.0})))
    }
}

// impl<T: Serialize> IntoRes<T> for Result<Data<T>, Error> {
impl<T: Serialize> IntoResult<T> for Result<T, Error> {
    fn into_result(self, req: &mut HttpRequest) -> Result<HttpResponse, ActixError> {
        match self {
            Ok(v) => Data(v).into_result(req),
            Err(e) => {
                let err = Error {
                    code: e.code,
                    msg: e.msg.clone(),
                    request_id: e.request_id,

                    status: e.status,
                    cause: None,
                    loc: None,
                };
                req.extensions_mut().insert(Trace::Error(e));
                Err(err.into())
            }
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
