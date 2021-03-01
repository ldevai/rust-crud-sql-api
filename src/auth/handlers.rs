use warp::Reply;
use serde_json::json;

use crate::{users, WebResult};
use crate::auth::create_jwt;
use crate::auth::models::{LoginRequest, LoginResponse, Role};
use crate::environment::Environment;
use crate::error::{AuthError};
use crate::users::models::{UserCreateRequest};

pub async fn register_handler(mut _req: UserCreateRequest, _env: Environment) -> WebResult<impl Reply> {
    match users::service::get_user_by_email(&_req.email, _env.db()).await {
        Ok(None) => (),
        Ok(existing) => {
            println!("[register_handler] User {} already exists", &existing.unwrap().email);
            return Ok(warp::reply::json(&json!({"status":"error", "message":"Unable to complete registration, email already registered"})))
        },
        _ => (),
    }

    let hash = _env.argon().hasher().with_password(&_req.password).hash().unwrap();
    _req.password = hash;
    _req.role = Some(Role::User);

    let email = _req.email.clone();
    match users::service::create_user(_req, _env.db()).await {
        Err(e) => {
            println!("[register_handler] Error registering user {}: {:?}", &email, e.message);
            return Ok(warp::reply::json(&json!({"status":"error", "message":"Registration error"})))
        },
        _ => {
            println!("[register_handler] Registration successful: {:?}", &email);
            return Ok(warp::reply::json(&json!({"status": "success"})));
        }
    }
}

pub async fn login_handler(_req: LoginRequest, _env: Environment) -> WebResult<impl Reply> {
    let user_option = match users::service::get_user_by_email(&_req.email, _env.db()).await {
        Ok(None) => {
            println!("[login_handler] Error authenticating user {:?}", &_req.email);
            return Err(warp::reject::custom(AuthError::InvalidCredentials))
            // return Ok(warp::reply::json(&json!({"status":"error", "message":"Email or password unknown"})));
        },
        Ok(existing) => existing,
        _ => {
            println!("[login_handler] Error authenticating user {:?}", &_req.email);
            return Ok(warp::reply::json(&json!({"status":"error", "message":"Email or password unknown"})));
        },
    };

    let user = user_option.unwrap();
    let is_valid = _env
        .argon()
        .verifier()
        .with_hash(&user.password)
        .with_password(&_req.password)
        .verify()
        .or(Err(warp::reject::custom(AuthError::ArgonError)))?;

    if !is_valid {
        println!("[login_handler] Invalid credentials for user {:?}", &_req.email);
        return Err(warp::reject::custom(AuthError::InvalidCredentials))
    }

    println!("[login_handler] Authenticated user '{}' ({})", &user.email, &user.role);
    let token = create_jwt(&user.id.to_string(), &Role::from_str(&user.role)).unwrap();
    let body = LoginResponse::from_user(user, token);
    return Ok(warp::reply::json(&body));
}
