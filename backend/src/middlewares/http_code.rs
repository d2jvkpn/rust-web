use actix_web::http::StatusCode;

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum HttpCode {
    #[error("no route")] // -1
    NoRoute,

    #[error("bad request")] // -2
    BadRequest,

    #[error("invalid token")] // -3
    InvalidToken,

    #[error("no changes")] // -4
    NoChanges,

    #[error("canceled")] // 1
    Canceled,

    #[error("unknown")] // 2
    Unknown,

    #[error("invalid argument")] // 3
    InvalidArgument,

    #[error("not found")] // 5
    NotFound,

    #[error("already exists")] // 6
    AlreadyExists,

    #[error("permission denied")] // 7
    PermissionDenied,

    #[error("resource exhausted")] // 8
    ResourceExhausted,

    #[error("aborted")] // 10
    Aborted,

    #[error("unexpected error")] // 13: internal server error
    UnexpectedError,

    #[error("database error")] // 13: internal server error
    DBError,

    // #[error("server error")] // 13: internal server error
    // ActixError,
    #[error("unauthenticated")] // 16
    Unauthenticated,
}

impl HttpCode {
    // grpc codes: go doc google.golang.org/grpc/codes.Internal
    pub fn code(&self) -> i32 {
        match self {
            Self::NoRoute => -1,
            Self::InvalidToken => -2,
            Self::BadRequest => -3,
            Self::NoChanges => -4,
            Self::Canceled => 1,
            Self::Unknown => 2,
            Self::InvalidArgument => 3,
            Self::NotFound => 5,
            Self::AlreadyExists => 6,
            Self::PermissionDenied => 7,
            Self::ResourceExhausted => 8,
            Self::Aborted => 10,
            Self::UnexpectedError | Self::DBError => 13,
            Self::Unauthenticated => 16,
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::NoRoute => StatusCode::BAD_REQUEST,
            Self::InvalidToken => StatusCode::BAD_REQUEST,
            Self::BadRequest => StatusCode::BAD_REQUEST,
            Self::NoChanges => StatusCode::BAD_REQUEST,
            Self::Canceled => StatusCode::NOT_ACCEPTABLE,
            Self::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidArgument => StatusCode::BAD_REQUEST,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::AlreadyExists => StatusCode::NOT_ACCEPTABLE,
            Self::PermissionDenied => StatusCode::FORBIDDEN,
            Self::ResourceExhausted => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Aborted => StatusCode::NOT_ACCEPTABLE,
            Self::UnexpectedError | Self::DBError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Unauthenticated => StatusCode::UNAUTHORIZED,
        }
    }
}
