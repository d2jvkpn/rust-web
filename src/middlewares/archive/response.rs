use actix_web::{
    error::{Error as ActixError, ResponseError},
    http::{header::HeaderMap, StatusCode},
    HttpResponse,
};
use serde::Serialize;
use serde_json::json;
use sqlx::error::Error as SQLxError;
use thiserror;

// type MyResult<T> = Result<Data<T>, Error>;
pub const OK_JSON: &str = r#"{"code":0,"msg":"ok","data":{}}"#;

// pub enum Result {
//     Ok(HttpResponse),
//     Err(Error),
// }
// from std::result::Result<T, Error> -> Result

//#[derive(Serialize)]
//pub struct ResponseOk<T> {
//    pub code: i32,
//    pub msg: String,
//    pub data: Option<T>,
//}

//impl<T> From<Data<T>> for ResponseOk<T> {
//    fn from(v: Data<T>) -> Self {
//        Self { code: 0, msg: "ok".into(), data: Some(v.0) }
//    }
//}

// response data
#[derive(Debug, Serialize)]
pub struct Data<T>(pub T);

impl<T: Serialize> From<Data<T>> for HttpResponse {
    fn from(v: Data<T>) -> Self {
        HttpResponse::Ok().json(json!({"code": 0, "msg": "ok", "data": v}))
    }
}

// response error
// TODO: log error, using thiserror
#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    // -1
    #[error("no route")]
    NoRoute,

    // 1
    #[error("canceled: {0}")]
    Canceled(String),

    // 2
    #[error("unknown")]
    Unknown,

    // 3
    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    // 5
    #[error("not found: {0}")]
    NotFound(String),

    // 6
    #[error("already exists")]
    AlreadyExists,

    // 7
    #[error("permission denied: {0}")]
    PermissionDenied(String),

    // 8
    #[error("resource exhausted")]
    ResourceExhausted,

    // 10
    #[error("aboort")]
    Aborted,

    // 13
    #[error("internal: {0}")]
    Internal(String),

    // 13 01
    #[error("database error")]
    DBError(SQLxError),

    // 13 02
    #[error("internal server error")]
    ActixError(ActixError),

    // 16
    #[error("unauthenticated: {0}")]
    Unauthenticated(String),

    // 1001
    #[error("no changes")]
    NoChanges,
}

//impl fmt::Display for Error {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "{}", self)
//    }
//}

impl Error {
    // grpc codes: go doc google.golang.org/grpc/codes.Internal
    fn code(&self) -> i32 {
        match self {
            Self::NoRoute => -1,
            Self::Canceled(_) => 1,
            Self::Unknown => 2,
            Self::InvalidArgument(_) => 3,
            Self::NotFound(_) => 5,
            Self::AlreadyExists => 6,
            Self::PermissionDenied(_) => 7,
            Self::ResourceExhausted => 8,
            Self::Aborted => 10,
            Self::Internal(_) => 13,
            Self::DBError(_) => 1301,
            Self::ActixError(_) => 1302,
            Self::Unauthenticated(_) => 16,
            Self::NoChanges => 1001,
        }
    }

    pub fn header_name() -> &'static str {
        "x-response-error"
    }

    pub fn header_value(&self) -> String {
        format!("{};{}", self.code(), self)
    }

    pub fn extract_from_headers(headers: &mut HeaderMap) -> Option<(i32, String)> {
        let value = headers.get(Self::header_name())?.to_str().ok()?;
        let mut iter = value.splitn(2, ';');

        let code = iter.next()?.parse::<i32>().ok()?;
        let msg = iter.next()?.to_string();

        headers.remove(Self::header_name());

        Some((code, msg))
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NoRoute => StatusCode::BAD_REQUEST,
            Self::Canceled(_) => StatusCode::NOT_ACCEPTABLE,
            Self::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidArgument(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::AlreadyExists => StatusCode::NOT_ACCEPTABLE,
            Self::PermissionDenied(_) => StatusCode::FORBIDDEN,
            Self::ResourceExhausted => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Aborted => StatusCode::NOT_ACCEPTABLE,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DBError(_) | Self::ActixError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Unauthenticated(_) => StatusCode::UNAUTHORIZED,
            Self::NoChanges => StatusCode::NOT_ACCEPTABLE,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .append_header((Error::header_name(), self.header_value()))
            .json(json!({"code": self.code(),"msg":format!("{}", self),"data":{}}))
    }
}

impl From<ActixError> for Error {
    fn from(err: ActixError) -> Self {
        // Self::ActixError(err.to_string())
        Self::ActixError(err)
    }
}

impl From<SQLxError> for Error {
    fn from(err: SQLxError) -> Self {
        // Self::DBError(err.to_string())
        Self::DBError(err)
    }
}
