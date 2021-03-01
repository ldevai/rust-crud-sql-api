use serde::{Deserialize, Serialize};

use crate::users::models::User;
use chrono::{DateTime, Utc};

// Parsed user from JWT session that is injected in authenticated handlers
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AuthUser {
    pub id: String,
    pub role: Role,
    pub login_at: DateTime<Utc>,
}

impl AuthUser {
    pub fn new(id: String, role: String) -> AuthUser {
        AuthUser {
            id: id,
            role: Role::from_str(&role),
            login_at: Utc::now(),
        }
    }
}

impl std::fmt::Display for AuthUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "{}", &self.id)
        }
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub id: uuid::Uuid,
    pub email: String,
    pub name: String,
    pub roles: Vec<String>,
    pub access_token: String,
}

impl LoginResponse {
    pub fn from_user(user: User, access_token: String) -> LoginResponse {
        return LoginResponse {
            id: user.id,
            email: user.email,
            name: user.name,
            roles: vec!(user.role),
            access_token,
        };
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub enum Role {
    User,
    Admin,
}

impl Role {
    pub fn from_str(role: &str) -> Role {
        match role {
            "Admin" => Role::Admin,
            _ => Role::User,
        }
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::User => write!(f, "User"),
            Role::Admin => write!(f, "Admin"),
        }
    }
}
