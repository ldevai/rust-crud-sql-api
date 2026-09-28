use tokio::signal::unix::{SignalKind, signal};
use warp::Filter;

use crate::environment::Environment;

mod articles;
mod auth;
mod environment;
mod error;
mod users;

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");
    let env = Environment::from_env().await;

    let health = warp::path!("health").and(warp::get()).map(|| "ok");
    let routes = health
        .or(auth::routes::routes(env.clone()))
        .or(users::routes::routes(env.clone()))
        .or(articles::routes::routes(env))
        .recover(error::handle_rejection)
        .with(warp::log::custom(|info| {
            println!(
                "{} {} {} {:?}",
                info.method(),
                info.path(),
                info.status(),
                info.elapsed()
            );
        }));

    println!("listening on :{port}");
    warp::serve(routes)
        .bind(([0, 0, 0, 0], port))
        .await
        .graceful(shutdown_signal())
        .run()
        .await;
}

/// `docker stop` sends SIGTERM; finish in-flight requests and exit.
async fn shutdown_signal() {
    let mut sigterm = signal(SignalKind::terminate()).expect("install SIGTERM handler");
    tokio::select! {
        _ = sigterm.recv() => {}
        _ = tokio::signal::ctrl_c() => {}
    }
}
