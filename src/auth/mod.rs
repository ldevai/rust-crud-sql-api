use argon2::Argon2;
use argon2::password_hash::phc::PasswordHash;
use argon2::password_hash::{PasswordHasher, PasswordVerifier};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode, get_current_timestamp,
};
use serde::{Deserialize, Serialize};

use crate::error::ApiError;

pub mod handlers;
pub mod middleware;
pub mod models;
pub mod routes;

const TOKEN_TTL: u64 = 24 * 60 * 60;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Role {
    User,
    Admin,
}

impl Role {
    pub fn as_str(self) -> &'static str {
        match self {
            Role::User => "User",
            Role::Admin => "Admin",
        }
    }

    /// Admins can do everything a user can.
    pub fn grants(self, required: Role) -> bool {
        self == Role::Admin || self == required
    }
}

impl TryFrom<String> for Role {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "User" => Ok(Role::User),
            "Admin" => Ok(Role::Admin),
            other => Err(format!("unknown role {other:?}")),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: Role,
    pub exp: u64,
}

pub struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
    validation: Validation,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        // Only HS256 is accepted, and `exp` must be present and in the future.
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_required_spec_claims(&["exp", "sub"]);
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
            validation,
        }
    }

    pub fn create_jwt(&self, user_id: &str, role: Role) -> Result<String, ApiError> {
        let claims = Claims {
            sub: user_id.to_string(),
            role,
            exp: get_current_timestamp() + TOKEN_TTL,
        };
        encode(&Header::new(Algorithm::HS256), &claims, &self.encoding).map_err(ApiError::internal)
    }

    pub fn verify_jwt(&self, token: &str) -> Result<Claims, ApiError> {
        decode::<Claims>(token, &self.decoding, &self.validation)
            .map(|data| data.claims)
            .map_err(|_| ApiError::unauthorized("invalid or expired token"))
    }
}

// Argon2 is deliberately slow; run it off the async worker threads.
pub async fn hash_password(password: String) -> Result<String, ApiError> {
    tokio::task::spawn_blocking(move || {
        Argon2::default()
            .hash_password(password.as_bytes())
            .map(|h| h.to_string())
    })
    .await
    .map_err(ApiError::internal)?
    .map_err(ApiError::internal)
}

pub async fn verify_password(password: String, hash: String) -> Result<bool, ApiError> {
    tokio::task::spawn_blocking(move || {
        let hash = PasswordHash::new(&hash).map_err(ApiError::internal)?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &hash)
            .is_ok())
    })
    .await
    .map_err(ApiError::internal)?
}
