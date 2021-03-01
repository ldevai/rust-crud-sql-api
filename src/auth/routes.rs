use warp::{Filter, Reply};
use warp::filters::BoxedFilter;

use crate::environment;
use crate::auth::handlers;
use crate::environment::Environment;

pub fn routes(_env: Environment) -> BoxedFilter<(impl Reply, )> {
    let login_route = warp::path!("api" / "auth" / "login")
        .and(warp::post())
        .and(warp::body::json())
        .and(environment::with_env(_env.clone()))
        .and_then(handlers::login_handler);

    let register_route = warp::path!("api" / "auth" / "register")
        .and(warp::post())
        .and(warp::body::json())
        .and(environment::with_env(_env.clone()))
        .and_then(handlers::register_handler);

    let routes = login_route.or(register_route);
    // let routes = login_route;
    routes.boxed()
}
