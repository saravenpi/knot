use crate::models::{Claims, User};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocket;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket_db_pools::{Connection, Database};
use std::env;
use uuid::Uuid;

const JWT_SECRET_ENV: &str = "JWT_SECRET";
const DEFAULT_JWT_SECRET: &str = "your-super-secret-jwt-key-change-this-in-production";

pub fn get_jwt_secret() -> String {
    env::var(JWT_SECRET_ENV).unwrap_or_else(|_| DEFAULT_JWT_SECRET.to_string())
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}

pub fn create_jwt_token(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(30))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        username: user.username.clone(),
        exp: expiration,
        iat: Utc::now().timestamp() as usize,
    };

    let secret = get_jwt_secret();
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn verify_jwt_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = get_jwt_secret();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub username: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = &'static str;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Check for Authorization header
        let auth_header = req.headers().get_one("Authorization");

        if let Some(auth_value) = auth_header {
            if let Some(token) = auth_value.strip_prefix("Bearer ") {
                match verify_jwt_token(token) {
                    Ok(claims) => {
                        if let Ok(user_id) = Uuid::parse_str(&claims.sub) {
                            return Outcome::Success(AuthenticatedUser {
                                user_id,
                                username: claims.username,
                            });
                        }
                    }
                    Err(_) => return Outcome::Error((Status::Unauthorized, "Invalid token")),
                }
            }
        }

        Outcome::Error((
            Status::Unauthorized,
            "Missing or invalid authorization header",
        ))
    }
}

#[derive(Database)]
#[database("knot_db")]
pub struct KnotDb(rocket_db_pools::sqlx::PgPool);

pub type DbPool = Connection<KnotDb>;
