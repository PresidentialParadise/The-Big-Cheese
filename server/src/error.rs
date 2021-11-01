#![allow(clippy::module_name_repetitions)]
use axum::{
    http::{Response, StatusCode},
    response::IntoResponse,
};
use thiserror::Error;
use tracing::{event, Level};


#[derive(Error, Debug)]
pub enum CheeseError {
    #[error("MongoDB encountered an error")]
    Mongo(#[from] mongodb::error::Error),
    #[error("Encountered an ObjectID error")]
    Oid(#[from] mongodb::bson::oid::Error),
}

impl IntoResponse for CheeseError {
    type Body = axum::body::Body;

    type BodyError = <Self::Body as axum::body::HttpBody>::Error;

    fn into_response(self) -> axum::http::Response<Self::Body> {
        let bb = match self {
            Self::Mongo(e) => {
                event!(Level::ERROR, "MongoDB error: {:?}", e);
                Self::Body::from("MongoDB did a fuckywucky")
            }
            Self::Oid(e) => {
                event!(Level::ERROR, "ObjectID error: {:?}", e);
                Self::Body::from("ObjectID did a fuckywucky")
            }
        };

        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(bb)
            .unwrap()
    }
}
