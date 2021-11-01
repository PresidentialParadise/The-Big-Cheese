use crate::auth::error::VerifyError;
use crate::auth::verify;
use crate::db_connection::DBClient;
use crate::error::CheeseError;
use crate::models::{Token, User};
use axum::async_trait;
use axum::body::Body;
use axum::extract::{FromRequest, RequestParts};
use axum::http::header::AUTHORIZATION;
use mongodb::bson::oid::ObjectId;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthorizationError {
    #[error("no authorization header in request")]
    NoAuthorizationHeader,

    #[error("no bearer prefix in authorization header")]
    NoBearerPrefix,

    #[error("parse token in authorization header")]
    ParseUuid(#[from] uuid::Error),

    #[error("verify")]
    Verify(#[from] VerifyError),

    #[error("admin status is required for this route")]
    NotAdmin,

    #[error("this route can only be accessed for your own user or if you are admin")]
    NotSelf,
}

#[must_use = "You must check whether this user actually is the user specified in the request"]
pub struct SelfAuth(User);
#[must_use = "You must check whether this user actually is the user specified in the request"]
pub struct SelfOrAdminAuth(User);
pub struct Auth(pub User);
pub struct AdminAuth(pub User);

impl SelfAuth {
    #[allow(unused)]
    pub fn user(&self, username: &str) -> Result<&User, AuthorizationError> {
        if self.0.username == username {
            Ok(&self.0)
        } else {
            Err(AuthorizationError::NotSelf)
        }
    }

    #[allow(unused)]
    pub fn user_by_id(&self, id: &ObjectId) -> Result<&User, AuthorizationError> {
        if self.0.id == Some(*id) {
            Ok(&self.0)
        } else {
            Err(AuthorizationError::NotSelf)
        }
    }
}

#[async_trait]
impl FromRequest<Body> for SelfAuth {
    type Rejection = CheeseError;

    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        let user = do_verify(req).await?;
        Ok(Self(user))
    }
}

impl SelfOrAdminAuth {
    #[allow(unused)]
    pub fn user(&self, username: &str) -> Result<&User, AuthorizationError> {
        if self.0.username == username || self.0.admin {
            Ok(&self.0)
        } else {
            Err(AuthorizationError::NotSelf)
        }
    }

    #[allow(unused)]
    pub fn user_by_id(&self, id: &ObjectId) -> Result<&User, AuthorizationError> {
        if self.0.id == Some(*id) || self.0.admin {
            Ok(&self.0)
        } else {
            Err(AuthorizationError::NotSelf)
        }
    }
}

#[async_trait]
impl FromRequest<Body> for SelfOrAdminAuth {
    type Rejection = CheeseError;

    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        let user = do_verify(req).await?;
        Ok(Self(user))
    }
}

#[async_trait]
impl FromRequest<Body> for Auth {
    type Rejection = CheeseError;

    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        let user = do_verify(req).await?;
        Ok(Self(user))
    }
}

#[async_trait]
impl FromRequest<Body> for AdminAuth {
    type Rejection = CheeseError;

    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        let user = do_verify(req).await?;
        if !user.admin {
            return Err(AuthorizationError::NotAdmin.into());
        }

        Ok(Self(user))
    }
}

async fn do_verify(request: &RequestParts) -> Result<User, AuthorizationError> {
    let auth_header = request
        .headers()
        .and_then(|headers| headers.get(AUTHORIZATION))
        .and_then(|value| value.to_str().ok())
        .ok_or(AuthorizationError::NoAuthorizationHeader)?;
    let token_string = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AuthorizationError::NoBearerPrefix)?;

    let token = token_string
        .parse()
        .map_err(AuthorizationError::ParseUuid)?;

    let client: &DBClient = request
        .extensions()
        .expect("extensions in request")
        .get()
        .expect("db client to be added to request at server start");

    let user = verify(&client.user_repo, &client.meta_repo, Token::new(token))
        .await
        .map_err(AuthorizationError::Verify)?;

    Ok(user)
}
