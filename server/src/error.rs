#![allow(clippy::module_name_repetitions)]
use axum::{
    http::{Response, StatusCode},
    response::IntoResponse,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CheeseError {
    #[error("MongoDB encountered an error")]
    Mongo(#[from] mongodb::error::Error),
}

impl IntoResponse for CheeseError {
    type Body = axum::body::Body;

    type BodyError = <Self::Body as axum::body::HttpBody>::Error;

    fn into_response(self) -> axum::http::Response<Self::Body> {
        let bb = match self {
            Self::Mongo(e) => {
                eprintln!("MongoDB error: {:?}", e);
                Self::Body::from("MongoDB did a fuckywucky")
            }
        };

        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(bb)
            .unwrap()
    }
}
