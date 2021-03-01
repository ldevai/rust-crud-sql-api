use warp::{Filter, Reply};
use warp::filters::BoxedFilter;

use crate::{auth, environment};
use crate::auth::models::Role;
use crate::environment::Environment;
use crate::users::handlers;

pub fn routes(_env: Environment) -> BoxedFilter<(impl Reply, )> {
    let get_users_route = warp::get().and(warp::path!("api" / "users")
        .and(environment::with_env(_env.clone()))
        .and(auth::middleware::with_auth(Role::Admin))
        .and_then(handlers::get_users_handler));

    let get_user_route = warp::get().and(warp::path!("api" / "users" / String)
        .and(environment::with_env(_env.clone()))
        .and(auth::middleware::with_auth(Role::Admin))
        .and_then(handlers::get_user_by_id_handler));

    let user_create_route = warp::post().and(warp::path!("api" / "users")
        .and(warp::body::json())
        .and(environment::with_env(_env.clone()))
        .and(auth::middleware::with_auth(Role::Admin))
        .and_then(handlers::user_create_handler));

    let user_update_route = warp::put().and(warp::path!("api" / "users")
        .and(warp::body::json())
        .and(environment::with_env(_env.clone()))
        .and(auth::middleware::with_auth(Role::Admin))
        .and_then(handlers::user_update_handler));

    let user_password_update_route = warp::put().and(warp::path!("api" / "users" / "changePassword")
        .and(warp::body::json())
        .and(environment::with_env(_env.clone()))
        .and(auth::middleware::authenticated())
        .and_then(handlers::password_update_handler));

    let routes = get_users_route.or(get_user_route)
        .or(user_create_route)
        .or(user_update_route)
        .or(user_password_update_route);

    routes.boxed()
}
