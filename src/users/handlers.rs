use uuid::Uuid;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

use crate::auth::models::AuthUser;
use crate::auth::{Role, hash_password, verify_password};
use crate::environment::Environment;
use crate::error::ApiError;
use crate::users::models::{
    PasswordUpdateRequest, UserCreateRequest, UserUpdateRequest, validate_password,
    validate_profile,
};
use crate::users::service;

fn user_not_found() -> ApiError {
    ApiError::not_found("user not found")
}

pub async fn get_users_handler(
    _admin: AuthUser,
    env: Environment,
) -> Result<impl Reply, Rejection> {
    let users = service::get_users(&env.db).await?;
    Ok(warp::reply::json(&users))
}

pub async fn get_user_by_id_handler(
    id: Uuid,
    _admin: AuthUser,
    env: Environment,
) -> Result<impl Reply, Rejection> {
    let user = service::get_user_by_id(&env.db, id)
        .await?
        .ok_or_else(user_not_found)?;
    Ok(warp::reply::json(&user))
}

/// Like registration, but an admin may pick the role (default `User`).
pub async fn user_create_handler(
    _admin: AuthUser,
    mut req: UserCreateRequest,
    env: Environment,
) -> Result<impl Reply, Rejection> {
    req.normalize();
    req.validate()?;
    let password_hash = hash_password(req.password.clone()).await?;
    let role = req.role.unwrap_or(Role::User);
    let user = service::create_user(&env.db, &req, &password_hash, role).await?;
    Ok(warp::reply::with_status(
        warp::reply::json(&user),
        StatusCode::CREATED,
    ))
}

/// Role changes reach the user's JWT at their next login.
pub async fn user_update_handler(
    _admin: AuthUser,
    mut req: UserUpdateRequest,
    env: Environment,
) -> Result<impl Reply, Rejection> {
    req.email = req.email.trim().to_lowercase();
    req.name = req.name.trim().to_string();
    validate_profile(&req.email, &req.name)?;
    let user = service::update_user(&env.db, &req)
        .await?
        .ok_or_else(user_not_found)?;
    Ok(warp::reply::json(&user))
}

pub async fn user_delete_handler(
    id: Uuid,
    _admin: AuthUser,
    env: Environment,
) -> Result<impl Reply, Rejection> {
    if !service::delete_user(&env.db, id).await? {
        return Err(user_not_found().into());
    }
    Ok(StatusCode::NO_CONTENT)
}

/// Users change their own password (current one required); admins can reset anyone's.
pub async fn password_update_handler(
    caller: AuthUser,
    req: PasswordUpdateRequest,
    env: Environment,
) -> Result<impl Reply, Rejection> {
    let own_password = caller.id == req.id;
    if !own_password && caller.role != Role::Admin {
        return Err(ApiError::forbidden("you can only change your own password").into());
    }
    validate_password(&req.new_password)?;

    let user = service::get_user_by_id(&env.db, req.id)
        .await?
        .ok_or_else(user_not_found)?;
    if own_password {
        let current = req.current_password.unwrap_or_default();
        if !verify_password(current, user.password_hash).await? {
            return Err(ApiError::forbidden("current password is incorrect").into());
        }
    }

    let password_hash = hash_password(req.new_password).await?;
    service::update_password(&env.db, req.id, &password_hash).await?;
    Ok(StatusCode::NO_CONTENT)
}
