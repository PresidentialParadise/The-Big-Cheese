use axum::response::IntoResponse;

#[allow(clippy::module_name_repetitions)]
mod recipes;
#[allow(clippy::module_name_repetitions)]
mod users;

pub use recipes::*;
pub use users::*;

pub async fn index() -> impl IntoResponse {
    "Hello World"
}
