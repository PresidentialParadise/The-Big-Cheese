#![allow(clippy::module_name_repetitions)]

use axum::{
    http::{Response, StatusCode},
    response::IntoResponse,
};
use thiserror::Error;
use tracing::{event, Level};

use crate::auth::error::{LoginError, RegisterError, VerifyError};
use crate::auth::middleware::AuthorizationError;

#[derive(Error, Debug)]
pub enum CheeseError {
    #[error("MongoDB encountered an error")]
    Mongo(#[from] mongodb::error::Error),

    #[error("Encountered an ObjectID error")]
    Oid(#[from] mongodb::bson::oid::Error),

    #[error("login")]
    Login(#[from] LoginError),
    #[error("register")]
    Register(#[from] RegisterError),

    #[error("hash password")]
    Hash(#[from] bcrypt::BcryptError),

    #[error("authorization")]
    Authorization(#[from] AuthorizationError),
}

/// `response_error` takes an error (something implementing Display), a literal string
/// (error message included in response) and a status code. The error is logged, and
/// response is built based on message and status. There is a special case for internal
/// server errors because of its frequent use.
macro_rules! response_error {
    ($e: expr, internal server error) => {
        response_error!(
            $e,
            "internal server error",
            StatusCode::INTERNAL_SERVER_ERROR
        )
    };

    ($e: expr, $message: literal, $code: expr) => {{
        event!(Level::ERROR, "{}", $e);

        Response::builder()
            .status($code)
            .body(Self::Body::from($message))
            .unwrap()
    }};
}

impl IntoResponse for CheeseError {
    type Body = axum::body::Body;

    type BodyError = <Self::Body as axum::body::HttpBody>::Error;

    fn into_response(self) -> axum::http::Response<Self::Body> {
        match self {
            Self::Mongo(e) => response_error!(e, internal server error),
            Self::Oid(e) => response_error!(e, internal server error),
            CheeseError::Login(l) => match l {
                e @ LoginError::InvalidCredentials => {
                    response_error!(e, "unauthorized", StatusCode::UNAUTHORIZED)
                }
                LoginError::Database(e) => response_error!(e, internal server error),
                LoginError::Bcrypt(e) => response_error!(e, internal server error),
                e @ LoginError::UserWithoutId => response_error!(e, internal server error),
            },
            CheeseError::Register(r) => match r {
                RegisterError::Database(e) => response_error!(e, internal server error),
                RegisterError::Bcrypt(e) => response_error!(e, internal server error),
                e @ RegisterError::UserExists => {
                    response_error!(e, "user exists", StatusCode::CONFLICT)
                }
            },
            CheeseError::Hash(e) => response_error!(e, internal server error),
            CheeseError::Authorization(a) => match a {
                e @ AuthorizationError::NoAuthorizationHeader => {
                    response_error!(e, "no authorization header", StatusCode::BAD_REQUEST)
                }
                e @ AuthorizationError::NoBearerPrefix => {
                    response_error!(e, "bad authorization header", StatusCode::BAD_REQUEST)
                }
                e @ AuthorizationError::NotFound => {
                    response_error!(e, "bad request", StatusCode::BAD_REQUEST)
                }
                e @ AuthorizationError::NotAdmin => response_error!(
                    e,
                    "admin status is required for this route",
                    StatusCode::UNAUTHORIZED
                ),
                e @ AuthorizationError::NotSelf => response_error!(
                    e,
                    "this route can only be ran as admin or when applying to yourself",
                    StatusCode::UNAUTHORIZED
                ),
                AuthorizationError::ParseUuid(e) => {
                    response_error!(e, "bad authorization header", StatusCode::BAD_REQUEST)
                }
                AuthorizationError::Verify(v) => match v {
                    e @ VerifyError::Expired => {
                        response_error!(e, "expired", StatusCode::FORBIDDEN)
                    }
                    e @ VerifyError::Invalid => {
                        response_error!(e, "forbidden", StatusCode::UNAUTHORIZED)
                    }
                    VerifyError::Database(e) => response_error!(e, internal server error),
                },
            },
        }
    }
}
