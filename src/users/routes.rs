use uuid::Uuid;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::auth::Role;
use crate::auth::middleware::{authenticated, with_auth};
use crate::environment::{Environment, json_body, with_env};
use crate::users::handlers;

pub fn routes(env: Environment) -> BoxedFilter<(impl Reply,)> {
    let admin = || with_auth(env.clone(), Role::Admin);

    let list = warp::path!("api" / "users")
        .and(warp::get())
        .and(admin())
        .and(with_env(env.clone()))
        .and_then(handlers::get_users_handler);

    let get = warp::path!("api" / "users" / Uuid)
        .and(warp::get())
        .and(admin())
        .and(with_env(env.clone()))
        .and_then(handlers::get_user_by_id_handler);

    let create = warp::path!("api" / "users")
        .and(warp::post())
        .and(admin())
        .and(json_body())
        .and(with_env(env.clone()))
        .and_then(handlers::user_create_handler);

    let update = warp::path!("api" / "users")
        .and(warp::put())
        .and(admin())
        .and(json_body())
        .and(with_env(env.clone()))
        .and_then(handlers::user_update_handler);

    let delete = warp::path!("api" / "users" / Uuid)
        .and(warp::delete())
        .and(admin())
        .and(with_env(env.clone()))
        .and_then(handlers::user_delete_handler);

    let change_password = warp::path!("api" / "users" / "changePassword")
        .and(warp::put())
        .and(authenticated(env.clone()))
        .and(json_body())
        .and(with_env(env.clone()))
        .and_then(handlers::password_update_handler);

    list.or(get)
        .or(create)
        .or(update)
        .or(delete)
        .or(change_password)
        .boxed()
}
