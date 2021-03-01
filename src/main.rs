use warp::Filter;

use crate::environment::Environment;

mod auth;
mod environment;
mod error;
mod users;
mod articles;

type Result<T> = std::result::Result<T, error::AppError>;
type WebResult<T> = std::result::Result<T, warp::reject::Rejection>;

#[tokio::main]
async fn main() {
    if dotenv::dotenv().is_err() {
        eprintln!("Error reading .env file in the current folder!");
    }

    let _env = match Environment::new().await {
        Ok(e) => e,
        Err(_e) => panic!("Unable to read environment configuration: {}", _e),
    };

    let auth_routes = auth::routes::routes(_env.clone());
    let user_routes = users::routes::routes(_env.clone());
    let article_routes = articles::routes::routes(_env.clone());
    let error_handler = error::handlers::error_handler;

    let routes = article_routes
        .or(auth_routes)
        .or(user_routes)
        .recover(error_handler);

    println!("Starting server on {}", _env.config().host);
    warp::serve(routes).run(_env.config().host).await;
}
