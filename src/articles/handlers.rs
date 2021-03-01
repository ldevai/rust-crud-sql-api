use warp::Reply;
use serde_json::json;

use crate::auth::models::AuthUser;
use crate::environment::Environment;
use crate::articles::service;
use crate::WebResult;
use crate::articles::models::{Article, Comment};

pub async fn get_article_by_url_handler(_url: String, _env: Environment) -> WebResult<impl Reply> {
    let _result = service::get_article_by_url(_url, _env.db()).await?;
    Ok(warp::reply::json(&_result))
}

pub async fn get_home_article_headers_handler(_env: Environment) -> WebResult<impl Reply> {
    let _result = service::get_home_article_headers(_env.db()).await?;
    Ok(warp::reply::json(&_result))
}

pub async fn get_article_headers_handler(_env: Environment) -> WebResult<impl Reply> {
    let _result = service::get_article_headers(_env.db()).await?;
    Ok(warp::reply::json(&_result))
}

pub async fn create_article_handler(mut _req: Article, _env: Environment, _user: AuthUser) -> WebResult<impl Reply> {
    _req.id = uuid::Uuid::new_v4().into();
    if _req.in_home == None {
        _req.in_home = Some(false);
    }
    let _result = service::create_article(&_req, _env.db()).await?;
    println!("[create_article_handler] Created article '{}'", &_req.title.unwrap());
    Ok(warp::reply::json(&json!({"status":"success", "message":"Article saved"})))
}

pub async fn update_article_handler(_req: Article, _env: Environment, _user: AuthUser) -> WebResult<impl Reply> {
    let mut original = service::get_article_by_id(_req.id.unwrap(), _env.db()).await?.unwrap();
    original.title = Some(_req.title.unwrap());
    original.content = Some(_req.content.unwrap());
    original.url = Some(_req.url.unwrap());
    original.in_home = Some(_req.in_home.unwrap());
    original.updated_at = Some(chrono::Utc::now());

    let _result = service::update_article(&original, _env.db()).await?;
    println!("[update_article_handler] id={}, title={}", &original.id.unwrap(), &original.title.unwrap());
    Ok(warp::reply::json(&json!({"status":"success", "message":"Article updated"})))
}

pub async fn delete_article_handler(_id: String, _env: Environment, _user: AuthUser) -> WebResult<impl Reply> {
    println!("[delete_article_handler] id={}", _id.clone());
    let _result = service::delete_article(&_id, _env.db()).await?;
    Ok(warp::reply::json(&json!({"status":"success", "message":"Article deleted"})))
}

pub async fn update_home_view_handler(_id: String, _env: Environment, _user: AuthUser) -> WebResult<impl Reply> {
    println!("[update_home_view_handler] id={}", &_id);
    let _result = service::update_home_view(_id, _env.db()).await?;
    Ok(warp::reply::json(&json!({"status":"success", "message":"Article updated"})))
}

// Comments

pub async fn get_article_comments_handler(_id: String, _env: Environment) -> WebResult<impl Reply> {
    let _result = service::get_comments(_id, _env.db()).await?;
    println!("[get_article_comments_handler] _result={:?}", _result);
    Ok(warp::reply::json(&_result))
}

pub async fn post_comment_handler(mut _req: Comment, _env: Environment) -> WebResult<impl Reply> {
    _req.id = uuid::Uuid::new_v4().into();
    let _result = service::create_comment(&_req, _env.db()).await?;
    println!("[post_comment_handler] article={}, email={}, name={}", _req.article_id, _req.email, _req.author);
    Ok(warp::reply::json(&json!({"status":"success", "message":"Comment saved"})))
}

