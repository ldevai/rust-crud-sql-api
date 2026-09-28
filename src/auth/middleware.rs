use warp::{Filter, Rejection};

use crate::auth::Role;
use crate::auth::models::AuthUser;
use crate::environment::Environment;
use crate::error::ApiError;

/// Any logged-in user.
pub fn authenticated(
    env: Environment,
) -> impl Filter<Extract = (AuthUser,), Error = Rejection> + Clone {
    with_auth(env, Role::User)
}

/// Reads `Authorization: Bearer <jwt>` and hands the caller to the handler:
/// 401 without a valid token, 403 when the token's role is not enough.
pub fn with_auth(
    env: Environment,
    required: Role,
) -> impl Filter<Extract = (AuthUser,), Error = Rejection> + Clone {
    let keys = env.keys;
    warp::header::optional::<String>("authorization").and_then(move |header: Option<String>| {
        let keys = keys.clone();
        async move {
            let token = header
                .as_deref()
                .and_then(|value| value.strip_prefix("Bearer "))
                .ok_or_else(|| ApiError::unauthorized("missing bearer token"))?;
            let claims = keys.verify_jwt(token)?;
            if !claims.role.grants(required) {
                return Err(
                    ApiError::forbidden(format!("requires role {}", required.as_str())).into(),
                );
            }
            let id = claims
                .sub
                .parse()
                .map_err(|_| ApiError::unauthorized("invalid or expired token"))?;
            Ok::<_, Rejection>(AuthUser {
                id,
                role: claims.role,
            })
        }
    })
}
