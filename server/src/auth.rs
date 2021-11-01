use thiserror::Error;
use crate::repository::Users;
use crate::models::{User, Token, DatedToken};

#[derive(Debug, Error)]
pub enum LoginError {
    #[error("invalid credentials provided")]
    InvalidCredentials,

    #[error(transparent)]
    Database(#[from] mongodb::error::Error),

    #[error(transparent)]
    Bcrypt(#[from] bcrypt::BcryptError),

    #[error("User queried without ID. This error is supposed to be unreachable.")]
    UserWithoutId,
}

#[derive(Debug, Error)]
pub enum RegisterError {
    #[error(transparent)]
    Database(#[from] mongodb::error::Error),

    #[error(transparent)]
    Bcrypt(#[from] bcrypt::BcryptError),

    #[error("user with this name already exists")]
    UserExists
}

#[derive(Debug, Error)]
pub enum VerifyError {
    #[error(transparent)]
    Database(#[from] mongodb::error::Error),

    #[error("token expired")]
    Expired,

    #[error("token invalid")]
    Invalid,
}

pub async fn verify(db: &Users, token: Token) -> Result<(), VerifyError> {
    let user = db.get_user_for_token(token).await?.ok_or(VerifyError::Invalid);

    Ok(())
}

pub async fn login(db: &Users, username: &str, password: &str) -> Result<Token, LoginError> {
    let mut user = db.get_user_by_name(username).await?.ok_or(LoginError::InvalidCredentials)?;

    if bcrypt::verify(password, &user.hashed_password)? {
        let token = DatedToken::generate();

        user.tokens.push(token);
        db.update_user(user.id.ok_or(LoginError::UserWithoutId)?, user).await?;

        Ok(token.token)
    } else {
        Err(LoginError::InvalidCredentials)
    }
}

pub async fn register(db: &Users, username: &str, password: &str) -> Result<(), RegisterError> {
    let password_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;

    if db.get_user_by_name(username).await?.is_some() {
        return Err(RegisterError::UserExists)
    }

    db.create_user(User {
        id: None,
        username: username.to_string(),
        display_name: username.to_string(),
        hashed_password: password_hash,
        admin: false,
        recipes: vec![],
        tokens: vec![],
    }).await?;

    Ok(())
}


