use gotham::handler::HandlerError;
use gotham::hyper::Method;
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::State;

use crate::HELLO_WORLD;

/// routes
/// /
/// /recipes
/// /user/<id>
pub fn router() -> Router {
    build_simple_router(|route| {
        // For the path "/" invoke the handler "say_hello"
        route
            .request(vec![Method::GET, Method::HEAD], "/")
            .to_async_borrowing(say_hello);
    })
}

async fn say_hello(_: &mut State) -> Result<&'static str, HandlerError> {
    Ok(HELLO_WORLD)
}
