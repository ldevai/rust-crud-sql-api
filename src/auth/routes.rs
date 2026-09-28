use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::auth::handlers;
use crate::environment::{Environment, json_body, with_env};

pub fn routes(env: Environment) -> BoxedFilter<(impl Reply,)> {
    let register = warp::path!("api" / "auth" / "register")
        .and(warp::post())
        .and(json_body())
        .and(with_env(env.clone()))
        .and_then(handlers::register_handler);

    let login = warp::path!("api" / "auth" / "login")
        .and(warp::post())
        .and(json_body())
        .and(with_env(env))
        .and_then(handlers::login_handler);

    register.or(login).boxed()
}
