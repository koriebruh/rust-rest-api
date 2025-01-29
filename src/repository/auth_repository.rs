use sqlx::mysql::MySqlPool;
use sqlx::Error;
use thiserror::Error;
use chrono::Utc;
use argon2::{self, Config};

use crate::domain::user::User;

#[derive(Debug,Error)]
pub enum ErrCustom {

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Username already exists")]
    UsernameExists,

    #[error("Email already exists")]
    EmailExists,

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Hashing error: {0}")]
    HashError(#[from] argon2::Error),

}

#[derive(Clone)]
pub(crate) struct AuthRepository {
    db: MySqlPool,
}

impl AuthRepository {
    pub fn new(db: MySqlPool) -> Self {
        Self { db }
    }

    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, ErrCustom> {
        argon2::verify_encoded(hash, password.as_bytes())
            .map_err(|e| ErrCustom::HashError(e))
    }
    fn hash_password(&self, password: &str) -> Result<String, ErrCustom> {
        let salt = rand::random::<[u8; 32]>();
        let config = Config::default();

        argon2::hash_encoded(
            password.as_bytes(),
            &salt,
            &config
        ).map_err(|e| ErrCustom::HashError(e))
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<bool, ErrCustom> {
        let row = sqlx::query!(
        "SELECT username, password FROM users WHERE username = ?",
        username
    )
            .fetch_optional(&self.db)
            .await?;

        match row {
            Some(row) => {
                let valid = self.verify_password(password, &row.password.unwrap_or_default())?;
                Ok(valid)
            }
            None => Ok(false)
        }
    }

    pub async fn register(&self, user: &User) -> Result<bool, ErrCustom> {
        // Gunakan method hash_password
        let hashed_password = self.hash_password(&user.password)?;

        let result = sqlx::query!(
            "INSERT INTO users (username, password, email, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
            user.username,
            hashed_password,
            user.email,
            user.created_at,
            user.updated_at
        )
            .execute(&self.db)
            .await;

        match result {
            Ok(_) => Ok(true),
            Err(_) => Ok(false)
        }
    }


}
