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
use sqlx::error::{DatabaseError, Error as SQLxError};
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
    pub fn into_req(self, req: &mut HttpRequest) -> ActixError {
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
    pub fn no_changes() -> Self {
        let mut res: Self = Error::NoChanges.into();
        res.loc = Some(loc!());
        res
    }

    #[track_caller]
    // fn invalid_token1<S: AsRef<str>>(msg: S) -> Self {
    pub fn invalid_token1(msg: String) -> Self {
        let mut res: Self = Error::InvalidToken(msg).into();
        res.loc = Some(loc!());
        res
    }

    #[track_caller]
    pub fn invalid_token2(e: AE, msg: String) -> Self {
        let mut res: Self = Error::InvalidToken(msg).into();
        res.cause = Some(e);
        res.loc = Some(loc!());
        res
    }

    #[track_caller]
    pub fn no_route() -> Self {
        let mut res: Self = Error::NoRoute.into();
        res.loc = Some(loc!());
        res
    }

    #[track_caller]
    pub fn canceled(msg: String) -> Self {
        let mut res: Self = Error::Canceled(msg).into();
        res.loc = Some(loc!());
        res
    }

    #[track_caller]
    pub fn unknown(e: AE) -> Self {
        let mut res: Self = Error::Unknown.into();
        res.cause = Some(e);
        res.loc = Some(loc!());
        res
    }

    #[track_caller]
    pub fn invalid1(msg: String) -> Self {
        let mut res: Self = Error::InvalidArgument(msg).into();
        res.loc = Some(loc!());
        res
    }

    #[track_caller]
    pub fn invalid2(e: AE, msg: String) -> Self {
        let mut res: Self = Error::InvalidArgument(msg).into();
        res.cause = Some(e);
        res.loc = Some(loc!());
        res
    }

    #[track_caller]
    pub fn not_found1(msg: String) -> Self {
        let mut res: Self = Error::NotFound(msg).into();
        res.loc = Some(loc!());
        res
    }

    #[track_caller]
    pub fn not_found2(e: AE, msg: String) -> Self {
        let mut res: Self = Error::NotFound(msg).into();
        res.cause = Some(e);
        res.loc = Some(loc!());
        res
    }

    #[track_caller]
    pub fn already_exists() -> Self {
        let mut res: Self = Error::AlreadyExists.into();
        res.loc = Some(loc!());
        res
    }

    #[track_caller]
    pub fn permission_denied(msg: String) -> Self {
        let mut res: Self = Error::PermissionDenied(msg).into();
        res.loc = Some(loc!());
        res
    }

    #[track_caller]
    pub fn resource_exhausted() -> Self {
        let mut res: Self = Error::ResourceExhausted.into();
        res.loc = Some(loc!());
        res
    }

    #[track_caller]
    pub fn aborted() -> Self {
        let mut res: Self = Error::Aborted.into();
        res.loc = Some(loc!());
        res
    }

    pub fn unexpected_error(e: AE) -> Self {
        let mut res: Self = Error::UnexpectedError.into();
        res.cause = Some(e);
        res.loc = Some(loc!());
        res
    }

    pub fn db_error(e: SQLxError) -> Self {
        let mut res: Self = Error::DBError.into();
        res.cause = Some(e.into());
        res.loc = Some(loc!());
        res
    }

    // TODO: more...
}

impl From<SQLxError> for Response {
    fn from(err: SQLxError) -> Self {
        // Self::DBError(err.to_string())
        let convert = |e: &Box<dyn DatabaseError>| {
            let code = match e.code() {
                Some(v) => v,
                None => return Error::DBError,
            };

            let v = if code.as_ref() == "23505" { Error::AlreadyExists } else { Error::DBError };
            v
        };

        let e2 = match &err {
            SQLxError::RowNotFound => Error::NotFound("...".to_string()),
            SQLxError::Database(e) => convert(e),
            _ => Error::DBError,
        };

        let mut res: Self = e2.into();
        res.cause = Some(err.into());
        res.loc = Some(loc!());
        res
    }
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
    pub fn new(data: T) -> Self {
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

    pub fn into_req(mut self, req: &mut HttpRequest) -> HttpResponse {
        let data = self.data.take();
        let request_id = self.response.request_id;
        req.extensions_mut().insert(self.response);
        HttpResponse::Ok()
            .json(json!({"code": 0,"msg":"ok", "requestId": request_id, "data": data}))
    }
}

impl<T: Serialize> From<T> for Data<T> {
    fn from(d: T) -> Self {
        Self::new(d)
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
