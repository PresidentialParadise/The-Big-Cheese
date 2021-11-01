use error::{LoginError, RegisterError, VerifyError};

use crate::models::{DatedToken, Token, User};
use crate::repository::{MetaRepo, Users};

pub mod error;
pub mod middleware;

/// Gets a user based on their token. Verifies that they are logged in and that their token is not
/// yet expired.
///
/// ```
/// # use big_cheese_server::test_util::test_db;
/// use big_cheese_server::auth::{register, login, verify};
/// # test_db(|client| async move {
///     # client.meta_repo.set_default_meta_if_not_exists().await.unwrap();
///
///     let users = &client.user_repo;
///     let meta = &client.meta_repo;
///
///     register(users, "test", "yeet").await.unwrap();
///     let token = login(users, "test", "yeet").await.unwrap();
///
///     let user = verify(users, meta, token).await.unwrap();
///     assert_eq!(user.username, "test");
/// # });
/// ```
///
/// # Expiration:
/// ```
/// # use big_cheese_server::test_util::test_db;
/// use big_cheese_server::auth::{register, login, verify};
/// use big_cheese_server::models::Config;
/// use std::time::Duration;
/// use std::thread;
/// use big_cheese_server::auth::error::VerifyError;
/// # test_db(|client| async move {
///     # client.meta_repo.set_default_meta_if_not_exists().await.unwrap();
///
///     let users = &client.user_repo;
///     let meta = &client.meta_repo;
///
///     meta.set_config(Config {expiration_time: Duration::from_millis(1)}).await.unwrap();
///
///     register(users, "test", "yeet").await.unwrap();
///     let token = login(users, "test", "yeet").await.unwrap();
///
///     // let the token expire
///     thread::sleep(Duration::from_millis(100));
///
///     assert!(matches!(verify(users, meta, token).await, Err(VerifyError::Expired)));
/// # });
/// ```
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
        users.remove_token(&user, dt).await?;

        return Err(VerifyError::Expired);
    }

    Ok(user)
}

/// Generate a login token given a user's username and password. The user should be registered.
///
/// ```
/// # use big_cheese_server::test_util::test_db;
/// use big_cheese_server::auth::{register, login};
/// # test_db(|client| async move {
///     let users = &client.user_repo;
///     // not yet registered
///     assert!(login(users, "test", "yeet").await.is_err());
///
///     register(users, "test", "yeet").await.unwrap();
///
///     let _token = login(users, "test", "yeet").await.unwrap();
/// # });
/// ```
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

/// Register a new user given their username and password. Hashes the password.
/// Created user is *not* an admin.
///
/// ```
/// # use big_cheese_server::test_util::test_db;
/// use big_cheese_server::auth::register;
/// # test_db(|client| async move {
///     let users = &client.user_repo;
///     register(users, "test", "yeet").await.unwrap();
/// # });
/// ```
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
