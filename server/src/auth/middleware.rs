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

    #[error("a user with this id does not exist")]
    NotFound,

    #[error("this route can only be accessed for your own user or if you are admin")]
    NotSelf,
}

/// Add this as a parameter to a route to get access to the currently logged in user.
/// This route is only available when that user is logged in.
///
/// To get the user out, you need to provide some information about the user that made the request.
/// This struct is useful for routes where a user may only make this request if that request applies
/// to themselves
///
/// WARNING: without calling either the [`user`] or [`user_by_id`] functions, proper authentication
/// does not occur.
#[must_use = "You must check whether this user actually is the user specified in the request"]
pub struct SelfAuth(User);

/// Add this as a parameter to a route to get access to the currently logged in user.
/// This route is only available when that user is logged in.
///
/// To get the user out, you need to provide some information about the user that made the request.
/// This struct is useful for routes where a user may only make this request if that request applies
/// to themselves. However, this route is also available for admins.
///
/// WARNING: without calling either the [`user`] or [`user_by_id`] functions, proper authentication
/// does not occur.
#[must_use = "You must check whether this user actually is the user specified in the request"]
pub struct SelfOrAdminAuth(User);

/// Add this as a parameter to a route to get access to the currently logged in user.
/// This route is only available when that user is logged in.
pub struct Auth(pub User);

/// Add this as a parameter to a route to get access to the currently logged in user.
/// This route is only available when that user is logged in and is an admin.
pub struct AdminAuth(pub User);

impl SelfAuth {
    /// Get the user inside this `SelfAuth` by providing their username.
    ///
    /// ```
    /// use big_cheese_server::auth::middleware::SelfAuth;
    /// # use big_cheese_server::models::User;
    /// # use mongodb::bson::oid::ObjectId;
    ///
    /// let id = ObjectId::new();
    ///
    /// let user = User {
    ///     id: Some(id),
    ///     username: "1".into(),
    ///     display_name: "2".into(),
    ///     hashed_password: "3".into(),
    ///     admin: false,
    ///
    ///     recipes: vec![],
    ///     tokens: vec![],
    /// };
    ///
    /// let sa = SelfAuth::new_for_test(user.clone());
    ///
    /// // wrong name
    /// assert!(sa.user("").is_err());
    ///
    /// // to get the user out again you must prove that this request was made to apply to that user.
    /// // in this case we know their username so we can use it as proof
    ///
    /// assert_eq!(sa.user(&user.username).unwrap(), &user)
    /// ```
    ///
    /// Even admins
    /// ```
    /// use big_cheese_server::auth::middleware::SelfAuth;
    /// # use big_cheese_server::models::User;
    /// # use mongodb::bson::oid::ObjectId;
    ///
    /// let id = ObjectId::new();
    ///
    /// let user = User {
    ///     id: Some(id),
    ///     username: "1".into(),
    ///     display_name: "2".into(),
    ///     hashed_password: "3".into(),
    ///     admin: true,
    ///
    ///     recipes: vec![],
    ///     tokens: vec![],
    /// };
    ///
    /// let sa = SelfAuth::new_for_test(user.clone());
    ///
    /// // wrong name and admin, but with SelfAuth still unauthorized
    /// assert!(sa.user("").is_err());
    /// ```
    #[allow(unused)]
    pub fn user(&self, username: &str) -> Result<&User, AuthorizationError> {
        if self.0.username == username {
            Ok(&self.0)
        } else {
            Err(AuthorizationError::NotSelf)
        }
    }

    /// Get the user inside this `SelfAuth` by providing their user id.
    ///
    /// ```
    /// use big_cheese_server::auth::middleware::SelfAuth;
    /// # use big_cheese_server::models::User;
    /// # use mongodb::bson::oid::ObjectId;
    ///
    /// let id = ObjectId::new();
    ///
    /// let user = User {
    ///     id: Some(id),
    ///     username: "1".into(),
    ///     display_name: "2".into(),
    ///     hashed_password: "3".into(),
    ///     admin: false,
    ///
    ///     recipes: vec![],
    ///     tokens: vec![],
    /// };
    ///
    /// let sa = SelfAuth::new_for_test(user.clone());
    ///
    /// // wrong id
    /// assert!(sa.user_by_id(&ObjectId::new()).is_err());
    ///
    /// // to get the user out again you must prove that this request was made to apply to that user.
    /// // in this case we know their user id so we can use it as proof
    ///
    /// assert_eq!(sa.user_by_id(&id).unwrap(), &user)
    /// ```
    ///
    /// ```
    /// use big_cheese_server::auth::middleware::SelfAuth;
    /// # use big_cheese_server::models::User;
    /// # use mongodb::bson::oid::ObjectId;
    ///
    /// let id = ObjectId::new();
    ///
    /// let user = User {
    ///     id: Some(id),
    ///     username: "1".into(),
    ///     display_name: "2".into(),
    ///     hashed_password: "3".into(),
    ///     admin: true,
    ///
    ///     recipes: vec![],
    ///     tokens: vec![],
    /// };
    ///
    /// let sa = SelfAuth::new_for_test(user.clone());
    ///
    /// // wrong id, but admin. However, with SelfAuth still unauthorized
    /// assert!(sa.user_by_id(&ObjectId::new()).is_err());
    /// ```
    #[allow(unused)]
    pub fn user_by_id(&self, id: &ObjectId) -> Result<&User, AuthorizationError> {
        if self.0.id == Some(*id) {
            Ok(&self.0)
        } else {
            Err(AuthorizationError::NotSelf)
        }
    }

    /// Construct a `SelfAuth` for use in tests (with a known user inside it).
    #[allow(unused)]
    pub fn new_for_test(user: User) -> Self {
        Self(user)
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
    /// Get the user inside this `SelfOrAdminAuth` by providing their username.
    ///
    /// ```
    /// use big_cheese_server::auth::middleware::SelfOrAdminAuth;
    /// # use big_cheese_server::models::User;
    /// # use mongodb::bson::oid::ObjectId;
    ///
    /// let id = ObjectId::new();
    ///
    /// let user = User {
    ///     id: Some(id),
    ///     username: "1".into(),
    ///     display_name: "2".into(),
    ///     hashed_password: "3".into(),
    ///     admin: false,
    ///
    ///     recipes: vec![],
    ///     tokens: vec![],
    /// };
    ///
    /// let sa = SelfOrAdminAuth::new_for_test(user.clone());
    ///
    /// // wrong name
    /// assert!(sa.user("").is_err());
    ///
    /// // to get the user out again you must prove that this request was made to apply to that user.
    /// // in this case we know their username so we can use it as proof
    ///
    /// assert_eq!(sa.user(&user.username).unwrap(), &user)
    /// ```
    ///
    /// Admins are always authorized though, even when the name is wrong
    /// ```
    /// use big_cheese_server::auth::middleware::SelfOrAdminAuth;
    /// # use big_cheese_server::models::User;
    /// # use mongodb::bson::oid::ObjectId;
    ///
    /// let id = ObjectId::new();
    ///
    /// let user = User {
    ///     id: Some(id),
    ///     username: "1".into(),
    ///     display_name: "2".into(),
    ///     hashed_password: "3".into(),
    ///     admin: true,
    ///
    ///     recipes: vec![],
    ///     tokens: vec![],
    /// };
    ///
    /// let sa = SelfOrAdminAuth::new_for_test(user.clone());
    ///
    /// // wrong name but not admin so authorized
    /// assert!(sa.user("").is_ok());
    /// ```
    #[allow(unused)]
    pub fn user(&self, username: &str) -> Result<&User, AuthorizationError> {
        if self.0.username == username || self.0.admin {
            Ok(&self.0)
        } else {
            Err(AuthorizationError::NotSelf)
        }
    }

    /// Get the user inside this `SelfOrAdminAuth` by providing their user id.
    ///
    /// ```
    /// use big_cheese_server::auth::middleware::SelfOrAdminAuth;
    /// # use big_cheese_server::models::User;
    /// # use mongodb::bson::oid::ObjectId;
    ///
    /// let id = ObjectId::new();
    ///
    /// let user = User {
    ///     id: Some(id),
    ///     username: "1".into(),
    ///     display_name: "2".into(),
    ///     hashed_password: "3".into(),
    ///     admin: false,
    ///
    ///     recipes: vec![],
    ///     tokens: vec![],
    /// };
    ///
    /// let sa = SelfOrAdminAuth::new_for_test(user.clone());
    ///
    /// // wrong id
    /// assert!(sa.user_by_id(&ObjectId::new()).is_err());
    ///
    /// // to get the user out again you must prove that this request was made to apply to that user.
    /// // in this case we know their user id so we can use it as proof
    ///
    /// assert_eq!(sa.user_by_id(&id).unwrap(), &user)
    /// ```
    ///
    /// ```
    /// use big_cheese_server::auth::middleware::SelfOrAdminAuth;
    /// # use big_cheese_server::models::User;
    /// # use mongodb::bson::oid::ObjectId;
    ///
    /// let id = ObjectId::new();
    ///
    /// let user = User {
    ///     id: Some(id),
    ///     username: "1".into(),
    ///     display_name: "2".into(),
    ///     hashed_password: "3".into(),
    ///     admin: true,
    ///
    ///     recipes: vec![],
    ///     tokens: vec![],
    /// };
    ///
    /// let sa = SelfOrAdminAuth::new_for_test(user.clone());
    ///
    /// // wrong id, but admin so authorized
    /// assert!(sa.user_by_id(&ObjectId::new()).is_ok());
    /// ```
    #[allow(unused)]
    pub fn user_by_id(&self, id: &ObjectId) -> Result<&User, AuthorizationError> {
        if self.0.id == Some(*id) || self.0.admin {
            Ok(&self.0)
        } else {
            Err(AuthorizationError::NotSelf)
        }
    }

    /// Construct a `SelfOrAdminAuth` for use in tests (with a known user inside it).
    #[allow(unused)]
    pub fn new_for_test(user: User) -> Self {
        Self(user)
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
        .trim()
        .strip_prefix("Bearer:")
        .ok_or(AuthorizationError::NoBearerPrefix)?
        .trim();

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
