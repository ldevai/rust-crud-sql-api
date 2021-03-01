use thiserror::Error;
use serde::{Deserialize, Serialize};

pub mod handlers;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("could not hash password")]
    ArgonError,
}
impl warp::reject::Reject for AuthError {}

#[derive(Error, Debug, Serialize)]
pub enum AppError {
    #[error("wrong credentials")]
    WrongCredentialsError,
    #[error("jwt token not valid")]
    JWTTokenError,
    #[error("jwt token creation failed")]
    JWTTokenCreationError,
    #[error("no auth header")]
    NoAuthHeaderError,
    #[error("invalid auth header")]
    InvalidAuthHeaderError,
    #[error("no permission")]
    NoPermissionError,
}
impl warp::reject::Reject for AppError {}

impl From<sqlx::error::Error> for AppError {
    fn from(_err: sqlx::error::Error) -> Self {
        AppError::WrongCredentialsError
    }
}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    message: String,
    status: String,
}

#[derive(Error, Debug)]
pub enum UserError {
    #[error("could not create user")]
    CreateError,
    #[error("could not update user")]
    UpdateError
}
impl warp::reject::Reject for UserError {}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DatabaseError {
    pub message: String,
}
impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "Database error"),
        }
    }
}
