use std::env;

use mongodb::{options::ClientOptions, Collection};

use crate::{models::Recipe, routes::router};
mod models;
mod repository;
mod routes;

#[tokio::main]
async fn main() {
    let client_uri = env::var("MONGODB_URI").expect("Missing MONGODB_URI env var");
    let db_name = env::var("MONGODB_NAME").expect("Missing MONGO_DB_NAME env var");

    let options = ClientOptions::parse(&client_uri).await.expect("oops");
    let client = mongodb::Client::with_options(options).expect("oops");
    let db = client.database(&db_name);

    let _recipes: Collection<Recipe> = db.collection("recipes");

    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);

    // All incoming requests are delegated to the router for further analysis and dispatch
    gotham::start(addr, router());
}

const HELLO_WORLD: &str = "Hello World!";
