use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::Role;
use crate::error::{ApiError, conflict_on_duplicate};
use crate::users::models::{User, UserCreateRequest, UserUpdateRequest};

const EMAIL_TAKEN: &str = "email already registered";

pub async fn get_users(db: &PgPool) -> Result<Vec<User>, ApiError> {
    let users = sqlx::query_as("SELECT * FROM users ORDER BY created_at")
        .fetch_all(db)
        .await?;
    Ok(users)
}

pub async fn get_user_by_id(db: &PgPool, id: Uuid) -> Result<Option<User>, ApiError> {
    let user = sqlx::query_as("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(db)
        .await?;
    Ok(user)
}

pub async fn get_user_by_email(db: &PgPool, email: &str) -> Result<Option<User>, ApiError> {
    let user = sqlx::query_as("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(db)
        .await?;
    Ok(user)
}

pub async fn create_user(
    db: &PgPool,
    req: &UserCreateRequest,
    password_hash: &str,
    role: Role,
) -> Result<User, ApiError> {
    sqlx::query_as(
        "INSERT INTO users (email, name, password_hash, role) VALUES ($1, $2, $3, $4) RETURNING *",
    )
    .bind(&req.email)
    .bind(&req.name)
    .bind(password_hash)
    .bind(role.as_str())
    .fetch_one(db)
    .await
    .map_err(conflict_on_duplicate(EMAIL_TAKEN))
}

pub async fn update_user(db: &PgPool, req: &UserUpdateRequest) -> Result<Option<User>, ApiError> {
    sqlx::query_as(
        "UPDATE users SET email = $1, name = $2, role = $3, updated_at = now() WHERE id = $4 RETURNING *",
    )
    .bind(&req.email)
    .bind(&req.name)
    .bind(req.role.as_str())
    .bind(req.id)
    .fetch_optional(db)
    .await
    .map_err(conflict_on_duplicate(EMAIL_TAKEN))
}

pub async fn update_password(db: &PgPool, id: Uuid, password_hash: &str) -> Result<(), ApiError> {
    sqlx::query("UPDATE users SET password_hash = $1, updated_at = now() WHERE id = $2")
        .bind(password_hash)
        .bind(id)
        .execute(db)
        .await?;
    Ok(())
}

pub async fn delete_user(db: &PgPool, id: Uuid) -> Result<bool, ApiError> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(db)
        .await?;
    Ok(result.rows_affected() > 0)
}
