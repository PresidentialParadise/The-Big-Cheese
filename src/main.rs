//! Provides a Restful web server managing recipes & users.
//!
//! API is currently:
//!
//! - `GET /`: return Hello World
//! - `GET /recipes`: return a JSON list of Recipes.
//! - `POST /recipes`: create a new Recipe.
//! - `GET /recipes/:id`: get a specific Recipe.
//! - `PUT /recipes/:id`: update a specific Recipe.
//! - `DELETE /recipes/:id`: delete a specific Recipe.
//! - `GET /users`: return a JSON list of user.
//! - `POST /users`: create a new User.
//! - `GET /users/:id`: get a specific User.
//! - `PUT /users/:id`: update a specific User.
//! - `DELETE /users/:id`: delete a specific User.

#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::unused_async)]
use std::{env, time::Duration};

use axum::{routing::get, Extension, Router};
use dotenv::dotenv;
use tower_http::cors::{Any, CorsLayer};

use std::net::SocketAddr;

use crate::db::DBClient;

mod db;
mod error;
mod handlers;
mod models;
mod repository;

#[allow(clippy::wildcard_imports)]
use handlers::*;

use tracing::{event, info, Level};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    if let Err(e) = dotenv() {
        event!(Level::WARN, "error finding dotenv file: {}. When not providing environment variables through a .env file this is normal", e);
    }

    let client_uri = env::var("DB_URI").expect("Missing DB_URI in .env");
    let db_name = env::var("DB_NAME").expect("Missing DB_NAME in .env");

    println!("{}", &client_uri);

    let client = DBClient::new(client_uri, &db_name)
        .await
        .expect("Failed to connect to mongodb client");

    let cors = CorsLayer::new()
        // .allow_credentials(true)
        .allow_headers(Any)
        .allow_methods(Any)
        .allow_origin(Any)
        .expose_headers(Any)
        .max_age(Duration::from_secs(60 * 60));

    // build our application with a route
    let app = Router::new()
        .route("/", get(index))
        .merge(user_routes())
        .merge(recipe_routes())
        .layer(Extension(client))
        .layer(cors);

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    info!("listening on http://localhost:{}", 8000);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
