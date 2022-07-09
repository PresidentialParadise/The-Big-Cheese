#![allow(clippy::module_name_repetitions)]
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use tracing::{event, Level};

#[derive(Error, Debug)]
pub enum CheeseError {
    #[error("Encountered a MongoDB error")]
    Mongo(#[from] mongodb::error::Error),
    #[error("Encountered an ObjectID error")]
    Oid(#[from] mongodb::bson::oid::Error),
    #[error("Encountered a Bcrypt error")]
    Bcrypt(#[from] bcrypt::BcryptError),
}

impl IntoResponse for CheeseError {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            Self::Mongo(e) => {
                event!(Level::ERROR, "MongoDB error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "MongoDB did a fuckywucky",
                )
            }
            Self::Oid(e) => {
                event!(Level::ERROR, "ObjectID error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "ObjectID did a fuckywucky",
                )
            }
            Self::Bcrypt(e) => {
                event!(Level::ERROR, "Bcrypt error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Bcrypt did a fuckywucky")
            }
        };

        let body = Json(json!({
            "error": err_msg,
        }));

        (status, body).into_response()
    }
}
