use sqlx::PgPool;
use uuid::Uuid;

use crate::articles::models::{Article, ArticleFields, ArticleSummary, Comment, NewComment};
use crate::error::{ApiError, conflict_on_duplicate};

const URL_TAKEN: &str = "an article with this url already exists";

pub async fn get_articles(db: &PgPool, home_only: bool) -> Result<Vec<ArticleSummary>, ApiError> {
    let articles = sqlx::query_as(
        "SELECT id, title, url, tags, in_home, created_at, updated_at FROM articles
         WHERE in_home OR NOT $1 ORDER BY created_at DESC",
    )
    .bind(home_only)
    .fetch_all(db)
    .await?;
    Ok(articles)
}

pub async fn get_article_by_url(db: &PgPool, url: &str) -> Result<Option<Article>, ApiError> {
    let article = sqlx::query_as("SELECT * FROM articles WHERE url = $1")
        .bind(url)
        .fetch_optional(db)
        .await?;
    Ok(article)
}

pub async fn create_article(db: &PgPool, article: &ArticleFields) -> Result<Article, ApiError> {
    sqlx::query_as(
        "INSERT INTO articles (title, url, content, tags, in_home) VALUES ($1, $2, $3, $4, $5) RETURNING *",
    )
    .bind(article.title.trim())
    .bind(&article.url)
    .bind(&article.content)
    .bind(&article.tags)
    .bind(article.in_home)
    .fetch_one(db)
    .await
    .map_err(conflict_on_duplicate(URL_TAKEN))
}

pub async fn update_article(
    db: &PgPool,
    id: Uuid,
    article: &ArticleFields,
) -> Result<Option<Article>, ApiError> {
    sqlx::query_as(
        "UPDATE articles SET title = $1, url = $2, content = $3, tags = $4, in_home = $5, updated_at = now()
         WHERE id = $6 RETURNING *",
    )
    .bind(article.title.trim())
    .bind(&article.url)
    .bind(&article.content)
    .bind(&article.tags)
    .bind(article.in_home)
    .bind(id)
    .fetch_optional(db)
    .await
    .map_err(conflict_on_duplicate(URL_TAKEN))
}

pub async fn delete_article(db: &PgPool, id: Uuid) -> Result<bool, ApiError> {
    let result = sqlx::query("DELETE FROM articles WHERE id = $1")
        .bind(id)
        .execute(db)
        .await?;
    Ok(result.rows_affected() > 0)
}

/// Flips `in_home` in one statement, so concurrent toggles cannot lose an update.
pub async fn toggle_home_view(db: &PgPool, id: Uuid) -> Result<Option<Article>, ApiError> {
    let article = sqlx::query_as(
        "UPDATE articles SET in_home = NOT in_home, updated_at = now() WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .fetch_optional(db)
    .await?;
    Ok(article)
}

pub async fn get_comments(db: &PgPool, article_id: Uuid) -> Result<Vec<Comment>, ApiError> {
    let comments = sqlx::query_as(
        "SELECT id, article_id, author, content, created_at FROM comments WHERE article_id = $1 ORDER BY created_at",
    )
    .bind(article_id)
    .fetch_all(db)
    .await?;
    Ok(comments)
}

pub async fn create_comment(db: &PgPool, comment: &NewComment) -> Result<Comment, ApiError> {
    sqlx::query_as(
        "INSERT INTO comments (article_id, author, email, content) VALUES ($1, $2, $3, $4)
         RETURNING id, article_id, author, content, created_at",
    )
    .bind(comment.article_id)
    .bind(comment.author.trim())
    .bind(comment.email.trim())
    .bind(&comment.content)
    .fetch_one(db)
    .await
    .map_err(|err| match &err {
        sqlx::Error::Database(e) if e.is_foreign_key_violation() => {
            ApiError::not_found("article not found")
        }
        _ => ApiError::internal(err),
    })
}

pub async fn delete_comment(
    db: &PgPool,
    article_id: Uuid,
    comment_id: Uuid,
) -> Result<bool, ApiError> {
    let result = sqlx::query("DELETE FROM comments WHERE id = $1 AND article_id = $2")
        .bind(comment_id)
        .bind(article_id)
        .execute(db)
        .await?;
    Ok(result.rows_affected() > 0)
}
