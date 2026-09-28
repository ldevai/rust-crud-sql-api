use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::error::ApiError;

/// An article in listings: everything but the content.
#[derive(Serialize, FromRow)]
pub struct ArticleSummary {
    pub id: Uuid,
    pub title: String,
    pub url: String,
    pub tags: Vec<String>,
    pub in_home: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, FromRow)]
pub struct Article {
    pub id: Uuid,
    pub title: String,
    pub url: String,
    pub content: String,
    pub tags: Vec<String>,
    pub in_home: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Body of `POST /api/articles`; `PUT` adds the `id`.
#[derive(Deserialize)]
pub struct ArticleFields {
    pub title: String,
    pub url: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub in_home: bool,
}

impl ArticleFields {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.title.trim().is_empty() {
            return Err(ApiError::bad_request("title is required"));
        }
        let url_ok = !self.url.is_empty()
            && self.url.len() <= 200
            && self
                .url
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
        if !url_ok {
            return Err(ApiError::bad_request("url must be a slug: a-z, 0-9 and -"));
        }
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct ArticleUpdate {
    pub id: Uuid,
    #[serde(flatten)]
    pub fields: ArticleFields,
}

/// Comments are public, so the commenter's email is stored but never returned.
#[derive(Serialize, FromRow)]
pub struct Comment {
    pub id: Uuid,
    pub article_id: Uuid,
    pub author: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct NewComment {
    pub article_id: Uuid,
    pub author: String,
    pub email: String,
    pub content: String,
}

impl NewComment {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.author.trim().is_empty() || self.content.trim().is_empty() {
            return Err(ApiError::bad_request("author and content are required"));
        }
        if !self.email.contains('@') {
            return Err(ApiError::bad_request("a valid email is required"));
        }
        Ok(())
    }
}
