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
use handlers::*;

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to read .env");

    let client_uri = env::var("DBURI").expect("Missing DB_URI in .env");
    let db_name = env::var("DBNAME").expect("Missing DB_NAME in .env");

    let client = DBClient::new(client_uri, &db_name)
        .await
        .expect("Failed to connect to mongodb client");

    // build our application with a route
    let app = Router::new()
        .route("/", get(index))
        .route("/recipes", get(recipes))
        .layer(AddExtensionLayer::new(client));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
