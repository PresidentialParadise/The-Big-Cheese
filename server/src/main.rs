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

#![allow(clippy::unused_async)]
use std::env;

use axum::{handler::get, AddExtensionLayer, Router};
use dotenv::dotenv;

use std::net::SocketAddr;

use crate::db_connection::DBClient;

mod db_connection;
mod error;
mod handlers;
mod models;
mod repository;
mod routes;

#[allow(clippy::wildcard_imports)]
use handlers::*;

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to read .env");

    let client_uri = env::var("DB_URI").expect("Missing DB_URI in .env");
    let db_name = env::var("DB_NAME").expect("Missing DB_NAME in .env");

    let client = DBClient::new(client_uri, &db_name)
        .await
        .expect("Failed to connect to mongodb client");

    // build our application with a route
    let app = Router::new()
        .route("/", get(index))
        .route("/recipes", get(fetch_recipes).post(push_recipe))
        .route(
            "/recipes/:id",
            get(fetch_recipe).patch(update_recipe).delete(delete_recipe),
        )
        .layer(AddExtensionLayer::new(client));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
