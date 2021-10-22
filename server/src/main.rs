use std::env;

use mongodb::options::ClientOptions;

use crate::routes::router;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    let client_uri = env::var("MONGODB_URI").expect("Missing MONGODB_URI env var");

    let options = ClientOptions::parse(&client_uri).await.expect("oops");
    let client = mongodb::Client::with_options(options).expect("oops");

    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);

    // All incoming requests are delegated to the router for further analysis and dispatch
    gotham::start(addr, router());
}

const HELLO_WORLD: &str = "Hello World!";
