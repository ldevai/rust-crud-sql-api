use warp::http::StatusCode;
use warp::{Rejection, Reply};

use crate::auth::models::{LoginRequest, LoginResponse};
use crate::auth::{Role, hash_password, verify_password};
use crate::environment::Environment;
use crate::error::ApiError;
use crate::users::models::UserCreateRequest;
use crate::users::service;

/// Self-registration always creates a `User`, whatever the body says.
pub async fn register_handler(
    mut req: UserCreateRequest,
    env: Environment,
) -> Result<impl Reply, Rejection> {
    req.normalize();
    req.validate()?;
    let password_hash = hash_password(req.password.clone()).await?;
    let user = service::create_user(&env.db, &req, &password_hash, Role::User).await?;
    Ok(warp::reply::with_status(
        warp::reply::json(&user),
        StatusCode::CREATED,
    ))
}

pub async fn login_handler(req: LoginRequest, env: Environment) -> Result<impl Reply, Rejection> {
    let invalid = || ApiError::unauthorized("invalid email or password");
    let email = req.email.trim().to_lowercase();
    let user = service::get_user_by_email(&env.db, &email)
        .await?
        .ok_or_else(invalid)?;
    if !verify_password(req.password, user.password_hash.clone()).await? {
        return Err(invalid().into());
    }
    let token = env.keys.create_jwt(&user.id.to_string(), user.role)?;
    Ok(warp::reply::json(&LoginResponse::new(user, token)))
}
