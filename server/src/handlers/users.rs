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
use serde::{Serialize, Deserialize};

use crate::{db_connection::DBClient, error::CheeseError, models::{User, UserList}, auth};
use crate::models::Token;

pub async fn fetch_users(
    Extension(db_client): Extension<DBClient>,
) -> Result<Json<UserList>, CheeseError> {
    let cursor = db_client.user_repo.get_all_users().await?;
    Ok(Json(UserList {
        users: cursor.try_collect().await?,
    }))
}

pub async fn fetch_user(
    Path(id): Path<String>,
    Extension(db_client): Extension<DBClient>,
) -> Result<Json<Option<User>>, CheeseError> {
    let res = db_client
        .user_repo
        .get_user_by_id(ObjectId::from_str(&id)?)
        .await?;
    Ok(Json(res))
}

pub async fn update_user(
    Path(id): Path<String>,
    Json(user): Json<User>,
    Extension(db_client): Extension<DBClient>,
) -> Result<Json<UpdateResult>, CheeseError> {
    let obj_id = ObjectId::from_str(&id)?;
    let res = db_client.user_repo.update_user(obj_id, user).await?;

    Ok(Json(res))
}

pub async fn delete_user(
    Path(id): Path<String>,
    Extension(db_client): Extension<DBClient>,
) -> Result<Json<DeleteResult>, CheeseError> {
    let res = db_client
        .user_repo
        .delete_user(ObjectId::from_str(&id)?)
        .await?;
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