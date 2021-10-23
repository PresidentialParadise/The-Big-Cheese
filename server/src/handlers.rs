use axum::{extract::Extension, response::IntoResponse, Json};

use crate::{db_connection::DBClient, error::CheeseError, models::Recipe};

use futures::stream::TryStreamExt;

pub async fn index() -> impl IntoResponse {
    "Hello World"
}

pub async fn recipes(
    Extension(db_client): Extension<DBClient>,
) -> Result<Json<Vec<Recipe>>, CheeseError> {
    let cursor = db_client.recipe_repo.get_all_recipes().await?;
    Ok(Json(cursor.try_collect().await?))
}
