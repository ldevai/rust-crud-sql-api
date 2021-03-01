use crate::auth::models::{Claims, Role};
use crate::error::AppError;
use crate::Result;

pub mod handlers;
pub mod middleware;
pub mod models;
pub mod routes;

const BEARER: &str = "Bearer ";
const JWT_SECRET: &[u8] = b"secret";

pub fn create_jwt(uid: &str, role: &Role) -> Result<String> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(259200))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: uid.to_owned(),
        role: role.to_string(),
        exp: expiration as usize,
    };
    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS512);
    jsonwebtoken::encode(&header, &claims, &jsonwebtoken::EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| AppError::JWTTokenCreationError)
}
