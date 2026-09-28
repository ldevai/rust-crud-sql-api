use uuid::Uuid;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::articles::handlers;
use crate::auth::Role;
use crate::auth::middleware::with_auth;
use crate::environment::{Environment, json_body, with_env};

pub fn routes(env: Environment) -> BoxedFilter<(impl Reply,)> {
    let admin = || with_auth(env.clone(), Role::Admin);

    let list = warp::path!("api" / "articles")
        .and(warp::get())
        .and(with_env(env.clone()))
        .and_then(handlers::get_articles_handler);

    let list_home = warp::path!("api" / "articles_home")
        .and(warp::get())
        .and(with_env(env.clone()))
        .and_then(handlers::get_home_articles_handler);

    let get = warp::path!("api" / "articles" / String)
        .and(warp::get())
        .and(with_env(env.clone()))
        .and_then(handlers::get_article_by_url_handler);

    let create = warp::path!("api" / "articles")
        .and(warp::post())
        .and(admin())
        .and(json_body())
        .and(with_env(env.clone()))
        .and_then(handlers::create_article_handler);

    let update = warp::path!("api" / "articles")
        .and(warp::put())
        .and(admin())
        .and(json_body())
        .and(with_env(env.clone()))
        .and_then(handlers::update_article_handler);

    let delete = warp::path!("api" / "articles" / Uuid)
        .and(warp::delete())
        .and(admin())
        .and(with_env(env.clone()))
        .and_then(handlers::delete_article_handler);

    let toggle_home = warp::path!("api" / "articles" / "updateHomeView" / Uuid)
        .and(warp::put())
        .and(admin())
        .and(with_env(env.clone()))
        .and_then(handlers::update_home_view_handler);

    let comments = warp::path!("api" / "articles" / "comments" / Uuid)
        .and(warp::get())
        .and(with_env(env.clone()))
        .and_then(handlers::get_comments_handler);

    let post_comment = warp::path!("api" / "articles" / "comments")
        .and(warp::post())
        .and(json_body())
        .and(with_env(env.clone()))
        .and_then(handlers::post_comment_handler);

    let delete_comment = warp::path!("api" / "articles" / "comments" / Uuid / Uuid)
        .and(warp::delete())
        .and(admin())
        .and(with_env(env.clone()))
        .and_then(handlers::delete_comment_handler);

    list.or(list_home)
        .or(get)
        .or(create)
        .or(update)
        .or(delete)
        .or(toggle_home)
        .or(comments)
        .or(post_comment)
        .or(delete_comment)
        .boxed()
}
