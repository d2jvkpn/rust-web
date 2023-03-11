macro_rules! loc {
    () => {{
        let caller = std::panic::Location::caller();
        format!("{}:{}", caller.file(), caller.line())
    }};
}

use super::http_code::HttpCode;
use actix_web::{
    error::Error as ActixError, http::StatusCode, HttpMessage, HttpRequest, HttpResponse,
    ResponseError,
};
use anyhow::Error as AE;
use derive_more::Display;
use serde::Serialize;
use serde_json::json;
use sqlx::error::Error as SQLxError; // DatabaseError
use uuid::Uuid;

#[derive(Serialize, Debug, Display)]
#[display(fmt = "code: {code}, msg: {msg:?}, loc: {loc:?}")]
pub struct Error {
    pub code: i32,
    pub msg: Option<String>,
    pub request_id: Uuid,

    #[serde(skip)]
    pub status: StatusCode,
    #[serde(skip)]
    pub cause: Option<AE>,
    #[serde(skip)]
    pub loc: Option<String>,
}

impl From<HttpCode> for Error {
    fn from(e: HttpCode) -> Self {
        Self {
            code: e.code(),
            msg: Some(format!("{}", e)),
            request_id: Uuid::new_v4(),

            status: e.status_code(),
            cause: None,
            loc: None,
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        self.status
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status).json(
            json!({"code": self.code,"msg":self.msg, "requestId": self.request_id, "data": {}}),
        )
    }
}

impl Error {
    pub fn into_req(self, req: &mut HttpRequest) -> ActixError {
        let err = Self {
            code: self.code,
            msg: self.msg.clone(),
            request_id: self.request_id,

            status: self.status,
            cause: None,
            loc: None,
        };
        req.extensions_mut().insert(self);
        err.into()
    }

    #[track_caller]
    pub fn no_changes() -> Self {
        let mut err: Self = HttpCode::NoChanges.into();
        err.loc = Some(loc!());
        err
    }

    #[track_caller]
    pub fn invalid_token0<S: AsRef<str>>(msg: S) -> Self {
        let mut err: Self = HttpCode::InvalidToken(msg.as_ref().to_string()).into();
        err.loc = Some(loc!());
        err
    }

    #[track_caller]
    pub fn invalid_token1(msg: String) -> Self {
        let mut err: Self = HttpCode::InvalidToken(msg).into();
        err.loc = Some(loc!());
        err
    }

    #[track_caller]
    pub fn invalid_token2(e: AE, msg: String) -> Self {
        let mut err: Self = HttpCode::InvalidToken(msg).into();
        err.cause = Some(e);
        err.loc = Some(loc!());
        err
    }

    #[track_caller]
    pub fn no_route() -> Self {
        let mut err: Self = HttpCode::NoRoute.into();
        err.loc = Some(loc!());
        err
    }

    #[track_caller]
    pub fn canceled(msg: String) -> Self {
        let mut err: Self = HttpCode::Canceled(msg).into();
        err.loc = Some(loc!());
        err
    }

    #[track_caller]
    pub fn unknown1() -> Self {
        let mut err: Self = HttpCode::Unknown.into();
        err.loc = Some(loc!());
        err
    }

    #[track_caller]
    pub fn unknown2(e: AE) -> Self {
        let mut err: Self = HttpCode::Unknown.into();
        err.cause = Some(e);
        err.loc = Some(loc!());
        err
    }

    #[track_caller]
    pub fn invalid1(msg: String) -> Self {
        let mut err: Self = HttpCode::InvalidArgument(msg).into();
        err.loc = Some(loc!());
        err
    }

    #[track_caller]
    pub fn invalid2(e: AE, msg: String) -> Self {
        let mut err: Self = HttpCode::InvalidArgument(msg).into();
        err.cause = Some(e);
        err.loc = Some(loc!());
        err
    }

    #[track_caller]
    pub fn not_found1(msg: String) -> Self {
        let mut err: Self = HttpCode::NotFound(msg).into();
        err.loc = Some(loc!());
        err
    }

    #[track_caller]
    pub fn not_found2(e: AE, msg: String) -> Self {
        let mut err: Self = HttpCode::NotFound(msg).into();
        err.cause = Some(e);
        err.loc = Some(loc!());
        err
    }

    #[track_caller]
    pub fn already_exists() -> Self {
        let mut err: Self = HttpCode::AlreadyExists.into();
        err.loc = Some(loc!());
        err
    }

    #[track_caller]
    pub fn permission_denied(msg: String) -> Self {
        let mut err: Self = HttpCode::PermissionDenied(msg).into();
        err.loc = Some(loc!());
        err
    }

    #[track_caller]
    pub fn resource_exhausted() -> Self {
        let mut err: Self = HttpCode::ResourceExhausted.into();
        err.loc = Some(loc!());
        err
    }

    #[track_caller]
    pub fn aborted() -> Self {
        let mut err: Self = HttpCode::Aborted.into();
        err.loc = Some(loc!());
        err
    }

    pub fn unexpected_error1() -> Self {
        let mut err: Self = HttpCode::UnexpectedError.into();
        err.loc = Some(loc!());
        err
    }

    pub fn unexpected_error2(e: AE) -> Self {
        let mut err: Self = HttpCode::UnexpectedError.into();
        err.cause = Some(e);
        err.loc = Some(loc!());
        err
    }

    pub fn db_error(e: SQLxError) -> Self {
        let mut err: Self = HttpCode::DBError.into();
        err.cause = Some(e.into());
        err.loc = Some(loc!());
        err
    }

    pub fn unauthenticated(msg: String) -> Self {
        let mut err: Self = HttpCode::Unauthenticated(msg).into();
        err.loc = Some(loc!());
        err
    }
}

/*
impl From<SQLxError> for Error {
    fn from(err: SQLxError) -> Self {
        // Self::DBError(err.to_string())
        let convert = |e: &Box<dyn DatabaseError>| {
            let code = match e.code() {
                Some(v) => v,
                None => return HttpCode::DBError,
            };

            if code.as_ref() == "23505" {
                HttpCode::AlreadyExists
            } else {
                HttpCode::DBError
            }
        };

        let e2 = match &err {
            SQLxError::RowNotFound => HttpCode::NotFound("...".to_string()),
            SQLxError::Database(e) => convert(e),
            _ => HttpCode::DBError,
        };

        let mut ae: Self = e2.into();
        ae.cause = Some(err.into());
        ae.loc = Some(loc!());
        ae
    }
}
*/

impl From<SQLxError> for Error {
    fn from(err: SQLxError) -> Self {
        let mut ae: Self = HttpCode::DBError.into();
        ae.cause = Some(err.into());
        ae.loc = Some(loc!());
        ae
    }
}
