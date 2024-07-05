use chrono::Utc;
use sqlx::{postgres::PgPool, query_as_unchecked, query_unchecked};
use warp::Rejection;

use crate::articles::models::{Article, Comment};

pub async fn get_article_by_id(_id: uuid::Uuid, connection: &PgPool) -> Result<Option<Article>, Rejection> {
    println!("[get_article_by_id] _id: {}", _id);
    let result = query_as_unchecked!(
        Article,
        r#"SELECT id, title, url, content, created_at, updated_at, in_home FROM articles WHERE id=$1"#,
        &_id
    )
        .fetch_one(connection)
        .await
        .map_err(|_e| { anyhow::Error::new(_e) })
        .ok();
    Ok(result)
}

pub async fn get_article_by_url(_url: String, connection: &PgPool) -> Result<Option<Article>, Rejection> {
    println!("[get_article_by_url] _url: {}", _url);
    let result = query_as_unchecked!(
        Article,
        r#"SELECT id, title, url, content, created_at, updated_at, in_home FROM articles WHERE url=$1"#,
        _url
    )
        .fetch_one(connection)
        .await
        .map_err(|_e| { anyhow::Error::new(_e) })
        .ok();
    Ok(result)
}

pub async fn get_home_article_headers(connection: &PgPool) -> Result<Option<Vec<Article>>, Rejection> {
    let result = query_as_unchecked!(
        Article,
        r#"SELECT id, title, url, '' as content, created_at, updated_at, in_home FROM articles WHERE in_home=true"#
    )
        .fetch_all(connection)
        .await
        .map_err(|_e| { anyhow::Error::new(_e) })
        .ok();
    Ok(result)
}


pub async fn get_article_headers(connection: &PgPool) -> Result<Option<Vec<Article>>, Rejection> {
    let result = query_as_unchecked!(
        Article,
        r#"SELECT id, title, url, '' as content, created_at, updated_at, in_home FROM articles"#
    )
        .fetch_all(connection)
        .await
        .map_err(|_e| { anyhow::Error::new(_e) })
        .ok();
    Ok(result)
}

pub async fn create_article(_article: &Article, connection: &PgPool) -> Result<Option<u64>, Rejection> {
    let _result = query_unchecked!(
        r#"INSERT INTO articles (id, title, url, content, created_at, in_home) VALUES ($1, $2, $3, $4, $5, $6)"#,
        _article.id,
        _article.title,
        _article.url,
        _article.content,
        Utc::now(),
        _article.in_home
    )
        .execute(connection)
        .await
        .unwrap();

    Ok(Some(0))
}

pub async fn update_article(_article: &Article, connection: &PgPool) -> Result<Option<u64>, Rejection> {
    query_unchecked!(
        r#"UPDATE articles SET title=$1, url=$2, content=$3, updated_at=$4, in_home=$5 WHERE id=$6"#,
        _article.title,
        _article.url,
        _article.content,
        _article.updated_at,
        _article.in_home,
        _article.id
    )
        .execute(connection)
        .await
        .unwrap();
    Ok(Some(0))
}

pub async fn delete_article(_id: &str, connection: &PgPool) -> Result<Option<u64>, Rejection> {
    query_as_unchecked!(
        Article,
        r#"DELETE FROM articles WHERE id=$1"#,
        uuid::Uuid::parse_str(&_id).unwrap()
    )
        .execute(connection)
        .await
        .unwrap();
    Ok(Some(0))
}

pub async fn update_home_view(_id: String, connection: &PgPool) -> Result<Option<u64>, Rejection> {
    let uuid = uuid::Uuid::parse_str(&_id).unwrap();
    query_unchecked!(
        r#"UPDATE articles SET in_home=(SELECT NOT in_home FROM articles WHERE id=$1) WHERE id=$2"#,
        uuid,
        uuid
    )
        .execute(connection)
        .await
        .unwrap();
    Ok(Some(0))
}



pub async fn get_comments(_id: String, connection: &PgPool) -> Result<Option<Vec<Comment>>, Rejection> {
    let result = query_as_unchecked!(
        Comment,
        r#"SELECT id, author, email, content, article_id, created_at, updated_at FROM comments WHERE article_id=$1"#,
        uuid::Uuid::parse_str(&_id).unwrap()
    )
        .fetch_all(connection)
        .await
        .map_err(|_e| { anyhow::Error::new(_e) })
        .ok();
    Ok(result)
}

pub async fn create_comment(_comment: &Comment, connection: &PgPool) -> Result<Option<u64>, Rejection> {
    let timestamp = Utc::now();
    let _result = query_unchecked!(
        r#"INSERT INTO comments (id, author, email, content, article_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
        _comment.id,
        _comment.author,
        _comment.email,
        _comment.content,
        _comment.article_id,
        timestamp,
        timestamp
    )
        .execute(connection)
        .await
        .unwrap();

    Ok(Some(0))
}
