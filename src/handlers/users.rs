use std::str::FromStr;

use axum::{
    extract::{Extension, Path},
    routing::get,
    Json, Router,
};
use futures::TryStreamExt;
use mongodb::{
    bson::oid::ObjectId,
    results::{DeleteResult, InsertOneResult, UpdateResult},
};

use crate::{
    db::DBClient,
    error::CheeseError,
    models::{User, UserList},
};

pub fn user_routes() -> Router {
    Router::new()
        .route("/users", get(fetch_users).post(push_user))
        .route(
            "/users/:id",
            get(fetch_user).patch(update_user).delete(delete_user),
        )
}

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
        .read_user(ObjectId::from_str(&id)?)
        .await?;
    Ok(Json(res))
}

pub async fn push_user(
    Json(user): Json<User>,
    Extension(db_client): Extension<DBClient>,
) -> Result<Json<InsertOneResult>, CheeseError> {
    let res = db_client.user_repo.create_user(user).await?;
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
