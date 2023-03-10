use actix_web::http::StatusCode;

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no changes")] // -3
    NoChanges,

    #[error("invalid token: {0}")] // -2
    InvalidToken(String),

    #[error("no route")] // -1
    NoRoute,

    #[error("canceled: {0}")] // 1
    Canceled(String),

    #[error("unknown")] // 2
    Unknown,

    #[error("invalid argument: {0}")] // 3
    InvalidArgument(String),

    #[error("not found: {0}")] // 5
    NotFound(String),

    #[error("already exists")] // 6
    AlreadyExists,

    #[error("permission denied: {0}")] // 7
    PermissionDenied(String),

    #[error("resource exhausted")] // 8
    ResourceExhausted,

    #[error("aboort")] // 10
    Aborted,

    #[error("unexpected error")] // 13: internal server error
    UnexpectedError,

    #[error("database error")] // 13: internal server error
    DBError,

    #[error("server error")] // 13: internal server error
    ActixError,

    #[error("unauthenticated: {0}")] // 16
    Unauthenticated(String),
}

impl Error {
    // grpc codes: go doc google.golang.org/grpc/codes.Internal
    pub fn code(&self) -> i32 {
        match self {
            Self::NoChanges => -3,
            Self::InvalidToken(_) => -2,
            Self::NoRoute => -1,
            Self::Canceled(_) => 1,
            Self::Unknown => 2,
            Self::InvalidArgument(_) => 3,
            Self::NotFound(_) => 5,
            Self::AlreadyExists => 6,
            Self::PermissionDenied(_) => 7,
            Self::ResourceExhausted => 8,
            Self::Aborted => 10,
            Self::UnexpectedError | Self::ActixError | Self::DBError => 13,
            Self::Unauthenticated(_) => 16,
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::NoChanges => StatusCode::BAD_REQUEST,
            Self::InvalidToken(_) => StatusCode::BAD_REQUEST,
            Self::NoRoute => StatusCode::BAD_REQUEST,
            Self::Canceled(_) => StatusCode::NOT_ACCEPTABLE,
            Self::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidArgument(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::AlreadyExists => StatusCode::NOT_ACCEPTABLE,
            Self::PermissionDenied(_) => StatusCode::FORBIDDEN,
            Self::ResourceExhausted => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Aborted => StatusCode::NOT_ACCEPTABLE,
            Self::UnexpectedError | Self::DBError | Self::ActixError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::Unauthenticated(_) => StatusCode::UNAUTHORIZED,
        }
    }
}
