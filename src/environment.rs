use std::convert::Infallible;
use std::sync::Arc;

use serde::de::DeserializeOwned;
use sqlx::PgPool;
use warp::{Filter, Rejection};

use crate::auth::Keys;

#[derive(Clone)]
pub struct Environment {
    pub db: PgPool,
    pub keys: Arc<Keys>,
}

impl Environment {
    pub async fn from_env() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let secret = std::env::var("AUTH_SECRET").expect("AUTH_SECRET must be set");

        let db = PgPool::connect(&database_url)
            .await
            .expect("cannot connect to Postgres");
        // Every statement is `IF NOT EXISTS`, so this is safe on every start.
        sqlx::raw_sql(include_str!("../schema.sql"))
            .execute(&db)
            .await
            .expect("cannot apply schema.sql");

        Self {
            db,
            keys: Arc::new(Keys::new(secret.as_bytes())),
        }
    }
}

pub fn with_env(
    env: Environment,
) -> impl Filter<Extract = (Environment,), Error = Infallible> + Clone {
    warp::any().map(move || env.clone())
}

/// A JSON request body of at most 64 KiB.
pub fn json_body<T: DeserializeOwned + Send>()
-> impl Filter<Extract = (T,), Error = Rejection> + Copy {
    warp::body::content_length_limit(64 * 1024).and(warp::body::json())
}
