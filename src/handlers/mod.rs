use axum::response::IntoResponse;

#[allow(clippy::module_name_repetitions)]
mod recipes;
#[allow(clippy::module_name_repetitions)]
mod users;

pub use recipes::*;
pub use users::*;

use crate::models::Details;

pub async fn index(user: Option<Details>) -> impl IntoResponse {
    match user {
        Some(u) => format!("Hey {}", u.display_name),
        None => "Welcome, unknown user!".to_string(),
    }
}
