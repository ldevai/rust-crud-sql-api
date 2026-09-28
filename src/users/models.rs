use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::auth::Role;
use crate::error::ApiError;

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    #[sqlx(try_from = "String")]
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct UserCreateRequest {
    pub email: String,
    pub name: String,
    pub password: String,
    pub role: Option<Role>,
}

impl UserCreateRequest {
    pub fn normalize(&mut self) {
        self.email = self.email.trim().to_lowercase();
        self.name = self.name.trim().to_string();
    }

    pub fn validate(&self) -> Result<(), ApiError> {
        validate_profile(&self.email, &self.name)?;
        validate_password(&self.password)
    }
}

#[derive(Deserialize)]
pub struct UserUpdateRequest {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: Role,
}

#[derive(Deserialize)]
pub struct PasswordUpdateRequest {
    pub id: Uuid,
    pub current_password: Option<String>,
    pub new_password: String,
}

pub fn validate_profile(email: &str, name: &str) -> Result<(), ApiError> {
    if !email.contains('@') || email.len() > 254 {
        return Err(ApiError::bad_request("a valid email is required"));
    }
    if name.is_empty() || name.len() > 150 {
        return Err(ApiError::bad_request("name must be 1-150 characters"));
    }
    Ok(())
}

pub fn validate_password(password: &str) -> Result<(), ApiError> {
    if !(8..=128).contains(&password.len()) {
        return Err(ApiError::bad_request("password must be 8-128 characters"));
    }
    Ok(())
}
