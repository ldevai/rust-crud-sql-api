use std::convert::Infallible;
use std::fmt::Display;

use serde_json::json;
use warp::filters::body::BodyDeserializeError;
use warp::http::StatusCode;
use warp::reject::{
    InvalidHeader, LengthRequired, MethodNotAllowed, PayloadTooLarge, UnsupportedMediaType,
};
use warp::{Rejection, Reply};

#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub message: String,
}

impl ApiError {
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, message)
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, message)
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::new(StatusCode::FORBIDDEN, message)
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, message)
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new(StatusCode::CONFLICT, message)
    }

    /// Logs the real cause and hides it from the client.
    pub fn internal(err: impl Display) -> Self {
        eprintln!("internal error: {err}");
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
    }
}

// warp converts any `Reject` type into a `Rejection`, so handlers can use `?`.
impl warp::reject::Reject for ApiError {}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        Self::internal(err)
    }
}

/// Turns a unique-constraint violation into `409 message`; anything else is a 500.
pub fn conflict_on_duplicate(message: &'static str) -> impl Fn(sqlx::Error) -> ApiError {
    move |err| match &err {
        sqlx::Error::Database(db) if db.is_unique_violation() => ApiError::conflict(message),
        _ => ApiError::internal(err),
    }
}

/// Every rejection — ours or warp's — becomes `{"error": "..."}` with the right status.
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (status, message) = if let Some(e) = err.find::<ApiError>() {
        (e.status, e.message.clone())
    } else if err.is_not_found() {
        (StatusCode::NOT_FOUND, "not found".to_string())
    } else if let Some(e) = err.find::<BodyDeserializeError>() {
        (StatusCode::BAD_REQUEST, e.to_string())
    } else if let Some(e) = err.find::<InvalidHeader>() {
        (StatusCode::BAD_REQUEST, e.to_string())
    } else if err.find::<UnsupportedMediaType>().is_some() {
        (
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "expected an application/json body".to_string(),
        )
    } else if err.find::<LengthRequired>().is_some() {
        (
            StatusCode::LENGTH_REQUIRED,
            "content-length header required".to_string(),
        )
    } else if err.find::<PayloadTooLarge>().is_some() {
        (
            StatusCode::PAYLOAD_TOO_LARGE,
            "request body too large".to_string(),
        )
    } else if err.find::<MethodNotAllowed>().is_some() {
        (
            StatusCode::METHOD_NOT_ALLOWED,
            "method not allowed".to_string(),
        )
    } else {
        eprintln!("unhandled rejection: {err:?}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal server error".to_string(),
        )
    };
    Ok(warp::reply::with_status(
        warp::reply::json(&json!({ "error": message })),
        status,
    ))
}
