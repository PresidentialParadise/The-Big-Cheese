#![allow(clippy::module_name_repetitions)]
use axum::{
    http::{Response, StatusCode},
    response::IntoResponse,
};
use thiserror::Error;
use tracing::{event, Level};
use crate::auth::{LoginError, RegisterError, VerifyError};

#[derive(Error, Debug)]
pub enum CheeseError {
    #[error("MongoDB encountered an error")]
    Mongo(#[from] mongodb::error::Error),

    #[error("login")]
    Login(#[from] LoginError),
    #[error("register")]
    Register(#[from] RegisterError),
    #[error("verify")]
    Verify(#[from] VerifyError),

    #[error("Encountered an ObjectID error")]
    Oid(#[from] mongodb::bson::oid::Error),
}

macro_rules! response_error {
    ($e: expr, internal server error) => {
        response_error!($e, "internal server error", StatusCode::INTERNAL_SERVER_ERROR)
    };

    ($e: expr, $message: literal, $code: expr) => {
        {
            event!(Level::ERROR, "{}", $e);

            Response::builder()
                .status($code)
                .body(Self::Body::from($message))
                .unwrap()
        }
    };
}

impl IntoResponse for CheeseError {
    type Body = axum::body::Body;

    type BodyError = <Self::Body as axum::body::HttpBody>::Error;

    fn into_response(self) -> axum::http::Response<Self::Body> {
        match self {
            Self::Mongo(e) => response_error!(e, internal server error),
            Self::Oid(e) => response_error!(e, internal server error),
            CheeseError::Login(l) => match l {
                e@LoginError::InvalidCredentials => response_error!(e, "unauthorized", StatusCode::UNAUTHORIZED),
                LoginError::Database(e) => response_error!(e, internal server error),
                LoginError::Bcrypt(e) => response_error!(e, internal server error),
                e@LoginError::UserWithoutId => response_error!(e, internal server error),
            }
            CheeseError::Register(r) => match r {
                RegisterError::Database(e) => response_error!(e, internal server error),
                RegisterError::Bcrypt(e) => response_error!(e, internal server error),
                e@RegisterError::UserExists => response_error!(e, "user exists", StatusCode::CONFLICT),
            }
            CheeseError::Verify(v) => match v {
                e@VerifyError::Expired => response_error!(e, "expired", StatusCode::FORBIDDEN),
                e@VerifyError::Invalid => response_error!(e, "forbidden", StatusCode::FORBIDDEN),
                VerifyError::Database(e) => response_error!(e, internal server error),
            }
        }
    }
}
