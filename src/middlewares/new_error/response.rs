#[macro_export]
macro_rules! loc {
    () => {{
        let caller = std::panic::Location::caller();
        format!("{}:{}", caller.file(), caller.line())
    }};
}

#[macro_export]
macro_rules! loc2 {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }

        // let caller = std::panic::Location::caller();
        let name = type_name_of(f);
        let list: Vec<&str> = name.split("::").collect();
        println!("??? {:?}", list);
        let length = list.len();
        let idx = if list[length - 2] == "{{closure}}" { length - 3 } else { length - 2 };

        format!("{}:{}:{}", file!(), line!(), list[idx])
    }};
}

use super::error::Error;
use actix_web::{
    error::Error as ActixError, http::StatusCode, HttpMessage, HttpRequest, HttpResponse,
    ResponseError,
};
use anyhow::Error as AE;
use derive_more::Display;
use serde::Serialize;
use serde_json::json;
use sqlx::error::Error as SQLxError;
use uuid::Uuid;

#[derive(Serialize, Debug, Display)]
#[display(fmt = "code: {code}, msg: {msg:?}, loc: {loc:?}")]
pub struct Response {
    pub code: i32,
    pub msg: Option<String>,
    pub request_id: Uuid,

    #[serde(skip)]
    pub status_code: StatusCode,
    #[serde(skip)]
    pub cause: Option<AE>,
    #[serde(skip)]
    pub loc: Option<String>,
}

impl From<Error> for Response {
    fn from(e: Error) -> Self {
        Self {
            code: e.code(),
            msg: Some(format!("{}", e)),
            request_id: Uuid::new_v4(),

            status_code: e.status_code(),
            cause: None,
            loc: None,
        }
    }
}

impl ResponseError for Response {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code).json(
            json!({"code": self.code,"msg":self.msg, "requestId": self.request_id, "data": {}}),
        )
    }
}

impl Response {
    fn into_req(self, req: &mut HttpRequest) -> ActixError {
        let res = Self {
            code: self.code,
            msg: self.msg.clone(),
            request_id: self.request_id,

            status_code: self.status_code,
            cause: None,
            loc: None,
        };
        req.extensions_mut().insert(self);
        res.into()
    }

    #[track_caller]
    fn no_changes() -> Self {
        let mut res: Self = Error::NoChanges.into();
        res.loc = Some(loc!());
        res
    }

    #[track_caller]
    fn invalid_token(e: AE, msg: String) -> Self {
        let mut res: Self = Error::InvalidToken(msg).into();
        res.cause = Some(e);
        res.loc = Some(loc!());
        res
    }

    fn unexpected_error(e: AE) -> Self {
        let mut res: Self = Error::UnexpectedError.into();
        res.cause = Some(e);
        res.loc = Some(loc!());
        res
    }

    fn db_error(e: SQLxError) -> Self {
        let mut res: Self = Error::DBError.into();
        res.cause = Some(e.into());
        res.loc = Some(loc!());
        res
    }

    fn actix_error(e: AE) -> Self {
        let mut res: Self = Error::ActixError.into();
        res.cause = Some(e.into());
        res.loc = Some(loc!());
        res
    }

    // TODO: more...
}

//
#[derive(Serialize, Debug)]
pub struct Data<T> {
    data: Option<T>,
    #[serde(flatten)]
    response: Response,
}

impl<T: Serialize> Data<T> {
    #[track_caller]
    fn new(data: T) -> Self {
        let response = Response {
            code: 0,
            msg: Some("ok".to_string()),
            request_id: Uuid::new_v4(),
            status_code: StatusCode::OK,
            cause: None,
            loc: Some(loc!()),
        };

        Self { data: Some(data), response }
    }

    fn into_req(mut self, req: &mut HttpRequest) -> HttpResponse {
        let data = self.data.take();
        let request_id = self.response.request_id;
        req.extensions_mut().insert(self.response);
        HttpResponse::Ok()
            .json(json!({"code": 0,"msg":"ok", "requestId": request_id, "data": data}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn t_response() {
        let err = Response::no_changes();
        println!("~~~ {:?}", err);
    }
}
