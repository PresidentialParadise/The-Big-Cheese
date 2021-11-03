use std::str::FromStr;

use axum::{
    extract::{Extension, Path},
    Json,
};
use futures::TryStreamExt;
use mongodb::{
    bson::oid::ObjectId,
    results::{DeleteResult, UpdateResult},
};
use serde::{Deserialize, Serialize};

use crate::auth::middleware::{AdminAuth, SelfOrAdminAuth};
use crate::models::Token;
use crate::{
    auth,
    db_connection::DBClient,
    error::CheeseError,
    models::{User, UserList},
};

pub async fn fetch_users(
    Extension(db_client): Extension<DBClient>,
    _auth: AdminAuth,
) -> Result<Json<UserList>, CheeseError> {
    let cursor = db_client.user_repo.get_all_users().await?;
    Ok(Json(UserList {
        users: cursor.try_collect().await?,
    }))
}

pub async fn fetch_user(
    Path(id): Path<String>,
    Extension(db_client): Extension<DBClient>,
    auth: SelfOrAdminAuth,
) -> Result<Json<Option<User>>, CheeseError> {
    let id = ObjectId::from_str(&id)?;

    auth.user_by_id(&id)?;

    let res = db_client.user_repo.get_user_by_id(id).await?;
    Ok(Json(res))
}

pub async fn update_user(
    Path(id): Path<String>,
    Json(user): Json<User>,
    Extension(db_client): Extension<DBClient>,
    auth: SelfOrAdminAuth,
) -> Result<Json<UpdateResult>, CheeseError> {
    let id = ObjectId::from_str(&id)?;

    auth.user_by_id(&id)?;

    let res = db_client.user_repo.update_user(id, user).await?;

    Ok(Json(res))
}

pub async fn delete_user(
    Path(id): Path<String>,
    Extension(db_client): Extension<DBClient>,
    auth: SelfOrAdminAuth,
) -> Result<Json<DeleteResult>, CheeseError> {
    let id = ObjectId::from_str(&id)?;

    auth.user_by_id(&id)?;

    let res = db_client.user_repo.delete_user(id).await?;
    Ok(Json(res))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiUser {
    username: String,
    password: String,
}

pub async fn register(
    Json(user): Json<ApiUser>,
    Extension(db_client): Extension<DBClient>,
) -> Result<Json<Token>, CheeseError> {
    auth::register(&db_client.user_repo, &user.username, &user.password).await?;

    let token = auth::login(&db_client.user_repo, &user.username, &user.password).await?;
    Ok(Json(token))
}

pub async fn login(
    Json(user): Json<ApiUser>,
    Extension(db_client): Extension<DBClient>,
) -> Result<Json<Token>, CheeseError> {
    let token = auth::login(&db_client.user_repo, &user.username, &user.password).await?;
    Ok(Json(token))
}
