use error::{LoginError, RegisterError, VerifyError};

use crate::models::{DatedToken, Token, User};
use crate::repository::{MetaRepo, Users};

pub mod error;
pub mod middleware;

pub async fn verify(users: &Users, meta: &MetaRepo, token: Token) -> Result<User, VerifyError> {
    let user = users
        .get_user_for_token(token)
        .await?
        .ok_or(VerifyError::Invalid)?;

    let config = meta.get_config().await?;

    let dt = user
        .tokens
        .iter()
        .find(|t| t.token == token)
        .ok_or(VerifyError::Invalid)?;

    // TODO: test
    if dt.expired(config.expiration_time) {
        return Err(VerifyError::Expired);
    }

    Ok(user)
}

pub async fn login(db: &Users, username: &str, password: &str) -> Result<Token, LoginError> {
    let mut user = db
        .get_user_by_name(username)
        .await?
        .ok_or(LoginError::InvalidCredentials)?;

    if bcrypt::verify(password, &user.hashed_password)? {
        let token = DatedToken::generate();

        user.tokens.push(token);
        db.update_user(user.id.ok_or(LoginError::UserWithoutId)?, user)
            .await?;

        Ok(token.token)
    } else {
        Err(LoginError::InvalidCredentials)
    }
}

pub async fn register(db: &Users, username: &str, password: &str) -> Result<(), RegisterError> {
    let password_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;

    if db.get_user_by_name(username).await?.is_some() {
        return Err(RegisterError::UserExists);
    }

    db.create_user(User {
        id: None,
        username: username.to_string(),
        display_name: username.to_string(),
        hashed_password: password_hash,
        admin: false,
        recipes: vec![],
        tokens: vec![],
    })
    .await?;

    Ok(())
}
