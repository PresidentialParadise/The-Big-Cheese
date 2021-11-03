use std::str::FromStr;

use axum::{
    extract::{Extension, Path},
    Json,
};
use futures::TryStreamExt;
use mongodb::{
    bson::oid::ObjectId,
    results::{DeleteResult, InsertOneResult, UpdateResult},
};

use crate::auth::middleware::Auth;
use crate::{db_connection::DBClient, error::CheeseError, models::Recipe};

pub async fn fetch_recipes(
    Extension(db_client): Extension<DBClient>,
) -> Result<Json<Vec<Recipe>>, CheeseError> {
    let cursor = db_client.recipe_repo.get_all_recipes().await?;

    Ok(Json(cursor.try_collect().await?))
}

pub async fn fetch_recipe(
    Path(id): Path<String>,
    Extension(db_client): Extension<DBClient>,
) -> Result<Json<Option<Recipe>>, CheeseError> {
    let res = db_client
        .recipe_repo
        .read_recipe(ObjectId::from_str(&id)?)
        .await?;
    Ok(Json(res))
}

pub async fn push_recipe(
    Json(recipe): Json<Recipe>,
    Extension(db_client): Extension<DBClient>,
    _user: Auth,
) -> Result<Json<InsertOneResult>, CheeseError> {
    let res = db_client.recipe_repo.create_recipe(recipe).await?;
    Ok(Json(res))
}

pub async fn update_recipe(
    Path(id): Path<String>,
    Json(recipe): Json<Recipe>,
    Extension(db_client): Extension<DBClient>,
) -> Result<Json<UpdateResult>, CheeseError> {
    let obj_id = ObjectId::from_str(&id)?;
    let res = db_client.recipe_repo.update_recipe(obj_id, recipe).await?;

    Ok(Json(res))
}

pub async fn delete_recipe(
    Path(id): Path<String>,
    Extension(db_client): Extension<DBClient>,
) -> Result<Json<DeleteResult>, CheeseError> {
    let res = db_client
        .recipe_repo
        .delete_recipe(ObjectId::from_str(&id)?)
        .await?;
    Ok(Json(res))
}
