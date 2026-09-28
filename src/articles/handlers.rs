use uuid::Uuid;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

use crate::articles::models::{ArticleFields, ArticleUpdate, NewComment};
use crate::articles::service;
use crate::auth::models::AuthUser;
use crate::environment::Environment;
use crate::error::ApiError;

fn article_not_found() -> ApiError {
    ApiError::not_found("article not found")
}

pub async fn get_articles_handler(env: Environment) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(
        &service::get_articles(&env.db, false).await?,
    ))
}

pub async fn get_home_articles_handler(env: Environment) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(
        &service::get_articles(&env.db, true).await?,
    ))
}

pub async fn get_article_by_url_handler(
    url: String,
    env: Environment,
) -> Result<impl Reply, Rejection> {
    let article = service::get_article_by_url(&env.db, &url)
        .await?
        .ok_or_else(article_not_found)?;
    Ok(warp::reply::json(&article))
}

pub async fn create_article_handler(
    _admin: AuthUser,
    req: ArticleFields,
    env: Environment,
) -> Result<impl Reply, Rejection> {
    req.validate()?;
    let article = service::create_article(&env.db, &req).await?;
    Ok(warp::reply::with_status(
        warp::reply::json(&article),
        StatusCode::CREATED,
    ))
}

pub async fn update_article_handler(
    _admin: AuthUser,
    req: ArticleUpdate,
    env: Environment,
) -> Result<impl Reply, Rejection> {
    req.fields.validate()?;
    let article = service::update_article(&env.db, req.id, &req.fields)
        .await?
        .ok_or_else(article_not_found)?;
    Ok(warp::reply::json(&article))
}

pub async fn delete_article_handler(
    id: Uuid,
    _admin: AuthUser,
    env: Environment,
) -> Result<impl Reply, Rejection> {
    if !service::delete_article(&env.db, id).await? {
        return Err(article_not_found().into());
    }
    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_home_view_handler(
    id: Uuid,
    _admin: AuthUser,
    env: Environment,
) -> Result<impl Reply, Rejection> {
    let article = service::toggle_home_view(&env.db, id)
        .await?
        .ok_or_else(article_not_found)?;
    Ok(warp::reply::json(&article))
}

pub async fn get_comments_handler(
    article_id: Uuid,
    env: Environment,
) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(
        &service::get_comments(&env.db, article_id).await?,
    ))
}

pub async fn post_comment_handler(
    req: NewComment,
    env: Environment,
) -> Result<impl Reply, Rejection> {
    req.validate()?;
    let comment = service::create_comment(&env.db, &req).await?;
    Ok(warp::reply::with_status(
        warp::reply::json(&comment),
        StatusCode::CREATED,
    ))
}

pub async fn delete_comment_handler(
    article_id: Uuid,
    comment_id: Uuid,
    _admin: AuthUser,
    env: Environment,
) -> Result<impl Reply, Rejection> {
    if !service::delete_comment(&env.db, article_id, comment_id).await? {
        return Err(ApiError::not_found("comment not found").into());
    }
    Ok(StatusCode::NO_CONTENT)
}
