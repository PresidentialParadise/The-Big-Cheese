#![allow(clippy::module_name_repetitions)]
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use tracing::{event, Level};

pub type CheeseResult<T> = Result<T, CheeseError>;
// pub type CheeseApiError = (StatusCode, Json<Value>);
// pub type CheeseApiResult<T> = Result<T, CheeseApiError>;

#[derive(Error, Debug)]
pub enum CheeseError {
    #[error(transparent)]
    Bcrypt(#[from] bcrypt::BcryptError),
    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    Mongo(#[from] mongodb::error::Error),
    #[error(transparent)]
    Oid(#[from] mongodb::bson::oid::Error),
    #[error("This username is already in use.")]
    UsernameExists,
    #[error("Missing credentials")]
    MissingCredentials,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Failed to generate token")]
    TokenCreation,
    #[error("Invalid token")]
    InvalidToken,
}

impl IntoResponse for CheeseError {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            Self::UsernameExists => (StatusCode::CONFLICT, self.to_string()),
            Self::InvalidCredentials => (StatusCode::UNAUTHORIZED, self.to_string()),
            Self::InvalidToken => (StatusCode::BAD_REQUEST, self.to_string()),
            _ => {
                event!(Level::ERROR, "{:?}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
        };

        let body = Json(json!({
            "error": err_msg,
        }));

        (status, body).into_response()
    }
}
