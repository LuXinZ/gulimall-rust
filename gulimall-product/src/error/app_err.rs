use utoipa::ToSchema;
use std::num::ParseFloatError;
use actix_web::ResponseError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use sea_orm::DbErr;
use crate::error::MessageResponse;


#[derive(Debug, thiserror::Error, ToSchema)]
pub enum AppError {
    #[error(transparent)]
    DbError(#[from] DbErr),
    #[error(transparent)]
    InvalidInput(#[from] validator::ValidationErrors),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    PermissionDenied(String),
    #[error("{0}")]
    UserBlocked(String),
    #[error("{0}")]
    AlreadyExists(String),
    #[error("{0}")]
    InvalidSession(String),
    #[error("{0}")]
    SessionNotExist(String),
    #[error("{0}")]
    Conflict(String),
    #[error("{0}")]
    UserNotActive(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    HashError(String),
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
    #[error(transparent)]
    ParseFloatError(#[from] ParseFloatError),
}
impl AppError {
    pub fn response(&self) -> MessageResponse {
        MessageResponse {
            message: self.error().to_string(),
            data: None,
            code: self.status_code().into(),
        }
    }

    pub fn error(&self) -> String {
        match self {
            Self::DbError(_) =>"db error".to_string(),
            Self::NotFound(_) => "NOT_FOUND".to_string(),
            Self::PermissionDenied(_) => "PERMISSION_DENIED".to_string(),
            Self::UserBlocked(_) => "USER_BLOCKED".to_string(),
            Self::AlreadyExists(_) => "ALREADY_EXISTS".to_string(),
            Self::SessionNotExist(_) => "SESSION_NOT_EXIST".to_string(),
            Self::InvalidSession(_) => "INVALID_SESSION".to_string(),
            Self::Conflict(message) => "Conflict".to_string(),

            Self::Unauthorized(_) => "UNAUTHORIZED".to_string(),
            Self::UserNotActive(_) => "USER_NOT_ACTIVE".to_string(),
            Self::InvalidInput(_) => "INVALID_INPUT".to_string(),

            Self::IoError(err) => {
                // tracing::error!("io error details: {err}");
                "IO_ERROR".to_string()
            }

            Self::HashError(err) => {
                // tracing::error!("hash error details: {err}");
                "HASH_ERROR".to_string()
            }

            Self::SerdeError(err) => {
                // tracing::error!("serde error details: {err}");
                "SERDE_ERROR".to_string()
            }
            Self::ParseFloatError(err) => {
                // tracing::error!("parse float number error details: {err}");
                "PARSE_FLOAT_ERROR".to_string()
            }
        }
    }
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::PermissionDenied(_) | Self::UserBlocked(_) => StatusCode::FORBIDDEN,
            Self::AlreadyExists(_) => StatusCode::from_u16(403).unwrap(),
            Self::Conflict(_) | Self::UserNotActive(_) => StatusCode::from_u16(409).unwrap(), // Conflict
            Self::InvalidInput(_) => StatusCode::from_u16(422).unwrap(), //Unprocessable Entity
            Self::Unauthorized(_)
            | Self::InvalidSession(_)
            | Self::SessionNotExist(_) => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        self.status_code()
    }
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        HttpResponse::build(status_code).json(self.response())
    }
}