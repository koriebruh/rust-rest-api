use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, EncodingKey};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::collections::HashSet;
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub : String, // what u want save, like id_user
    company: String,
    exp : usize,
    jti: String, // Token diinvalidasi dengan menyimpan jti-nya di blacklist
}

// Blacklist untuk menyimpan token yang sudah diinvalidasi
lazy_static! {
    static ref BLACKLISTED_TOKENS: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

const SECRET_KEY: &[u8] = b"my_secret_key";
const COMPANY: &str = "Korium"; // my company name in future before 2030 and will be big after 2030 add run in blockchain tech

fn generate_jti() -> String {
    use uuid::Uuid;
    Uuid::new_v4().to_string()
}

pub async fn create_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24)) // lifetime token
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        company: COMPANY.to_string(),
        exp: expiration as usize,
        jti: generate_jti(),
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY))
}
pub async fn get_claims_from_jwt(token: &str) -> Option<Claims> {
    // Cek apakah token ada di blacklist
    if is_token_blacklisted(token) {
        return None;
    }

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::default(),
    ) {
        Ok(token_data) => {
            // Cek waktu kadaluarsa
            let now = Utc::now().timestamp() as usize;
            if token_data.claims.exp <= now {
                return None;
            }
            Some(token_data.claims)
        }
        Err(_) => None,
    }
}

// BUAT AGAR KADALUARSA JWT
pub async fn invalidate_token(token: &str) -> bool {
    if let Ok(token_data) = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::default(),
    ) {
        if let Ok(mut blacklist) = BLACKLISTED_TOKENS.lock() {
            blacklist.insert(token_data.claims.jti);
            return true;
        }
    }
    false
}

fn is_token_blacklisted(token: &str) -> bool {
    if let Ok(token_data) = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::default(),
    ) {
        if let Ok(blacklist) = BLACKLISTED_TOKENS.lock() {
            return blacklist.contains(&token_data.claims.jti);
        }
    }
    false
}