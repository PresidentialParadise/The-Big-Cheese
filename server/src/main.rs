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
use std::env;

use axum::{
    routing::{get, post},
    AddExtensionLayer, Router,
};
use dotenv::dotenv;

use std::net::SocketAddr;

use crate::db_connection::DBClient;

mod auth;
mod db_connection;
mod error;
mod handlers;
mod models;
mod repository;

pub mod test_util;

#[allow(clippy::wildcard_imports)]
use handlers::*;

use rand::prelude::IteratorRandom;
use tracing::{event, Level};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    if let Err(e) = dotenv() {
        event!(Level::WARN, "error finding dotenv file: {}. When not providing environment variables through a .env file this is normal", e);
    }

    let client_uri = env::var("DB_URI").expect("Missing DB_URI in .env");
    let db_name = env::var("DB_NAME").expect("Missing DB_NAME in .env");
    let admin_user_name = env::var("ADMIN_USER_NAME").unwrap_or_else(|_| "admin".to_string());
    let admin_user_password = env::var("ADMIN_USER_PASSWORD").unwrap_or_else(|_| random_password());

    let client = DBClient::new(client_uri, &db_name)
        .await
        .expect("Failed to connect to mongodb client");

    client
        .meta_repo
        .set_default_meta_if_not_exists()
        .await
        .expect("couldn't set default meta");
    client
        .user_repo
        .first_user(&admin_user_name, &admin_user_password)
        .await
        .expect("couldn't create initial user");

    // build our application with a route
    let app = Router::new()
        .route("/", get(index))
        .route("/recipes", get(fetch_recipes).post(push_recipe))
        .route(
            "/recipes/:id",
            get(fetch_recipe).patch(update_recipe).delete(delete_recipe),
        )
        .route("/users", get(fetch_users))
        .route(
            "/users/:id",
            get(fetch_user).patch(update_user).delete(delete_user),
        )
        .route("/users/register", post(register))
        .route("/users/login", post(login))
        .layer(AddExtensionLayer::new(client));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    event!(Level::INFO, "listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn random_password() -> String {
    const PASSWORD_ALPHABET: &str =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    let mut pw = String::new();
    let mut rng = rand::thread_rng();

    for _ in 0..16 {
        pw.push(PASSWORD_ALPHABET.chars().choose(&mut rng).unwrap());
    }
    pw
}
