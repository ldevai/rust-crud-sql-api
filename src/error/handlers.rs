use std::convert::Infallible;

use warp::{http::StatusCode, Reply};

use crate::error::{AppError, AuthError, ErrorResponse, UserError};

pub async fn error_handler(err: warp::reject::Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())

    } else if let Some(e) = err.find::<UserError>() {
        println!("[error_handler] USER ERROR");
        match e {
            UserError::CreateError => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            _ => (StatusCode::BAD_REQUEST, e.to_string()),
        }

    } else if let Some(e) = err.find::<AppError>() {
        match e {
            AppError::WrongCredentialsError => (StatusCode::FORBIDDEN, e.to_string()),
            AppError::NoPermissionError => (StatusCode::UNAUTHORIZED, e.to_string()),
            AppError::JWTTokenError => (StatusCode::UNAUTHORIZED, e.to_string()),
            AppError::JWTTokenCreationError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
            _ => (StatusCode::BAD_REQUEST, e.to_string()),
        }

    } else if let Some(e) = err.find::<AuthError>() {
        match e {
            AuthError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Authentication failed".to_string()),
            _ => (StatusCode::BAD_REQUEST, e.to_string()),
        }

    } else if let Some(err)  = err.find::<warp::reject::MethodNotAllowed>() {
        println!("[error_handler] method not allowed");
        (
            StatusCode::METHOD_NOT_ALLOWED,
            err.to_string(),
        )
    } else if let Some(e) = err.find::<AuthError>() {
        println!("AUTH FAILED! {:?}", err);
        match e {
            AuthError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Authentication failed".to_string()),
            _ => (StatusCode::BAD_REQUEST, e.to_string())
        }
    } else {
        println!("[error_handler] unhandled: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    let json = warp::reply::json(&ErrorResponse {
        status: code.to_string(),
        message,
    });

    Ok(warp::reply::with_status(json, code))
}
