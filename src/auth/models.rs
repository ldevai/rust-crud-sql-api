use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::Role;
use crate::users::models::User;

/// The caller, as proven by a valid JWT. Injected into authenticated handlers.
#[derive(Clone, Copy, Debug)]
pub struct AuthUser {
    pub id: Uuid,
    pub role: Role,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: Role,
    pub access_token: String,
}

impl LoginResponse {
    pub fn new(user: User, access_token: String) -> Self {
        Self {
            id: user.id,
            email: user.email,
            name: user.name,
            role: user.role,
            access_token,
        }
    }
}
