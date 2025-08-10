use crate::auth::{create_jwt_token, hash_password, verify_password, AuthenticatedUser, DbPool};
use crate::models::{AuthResponse, CreateUserRequest, LoginRequest, User, UserProfile};
use chrono::Utc;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, post};
use uuid::Uuid;
use validator::Validate;

#[post("/register", data = "<request>")]
pub async fn register(
    mut db: DbPool,
    request: Json<CreateUserRequest>,
) -> Result<Json<AuthResponse>, Status> {
    // Validate input
    if let Err(_) = request.validate() {
        return Err(Status::BadRequest);
    }

    // Check if username or email already exists
    let existing_user = sqlx::query!(
        "SELECT id FROM users WHERE username = $1 OR email = $2",
        request.username,
        request.email
    )
    .fetch_optional(&mut **db)
    .await;

    match existing_user {
        Ok(Some(_)) => return Err(Status::Conflict),
        Ok(None) => {}
        Err(_) => return Err(Status::InternalServerError),
    }

    // Hash password
    let password_hash = match hash_password(&request.password) {
        Ok(hash) => hash,
        Err(_) => return Err(Status::InternalServerError),
    };

    // Create user
    let user_id = Uuid::new_v4();
    let now = Utc::now();

    let result = sqlx::query!(
        r#"
        INSERT INTO users (id, username, email, password_hash, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, username, email, created_at
        "#,
        user_id,
        request.username,
        request.email,
        password_hash,
        now,
        now
    )
    .fetch_one(&mut **db)
    .await;

    match result {
        Ok(row) => {
            let user = User {
                id: user_id,
                username: request.username.clone(),
                email: request.email.clone(),
                password_hash,
                created_at: now,
                updated_at: now,
            };

            let token = match create_jwt_token(&user) {
                Ok(token) => token,
                Err(_) => return Err(Status::InternalServerError),
            };

            let user_profile = UserProfile {
                id: user_id,
                username: request.username.clone(),
                email: request.email.clone(),
                created_at: now,
            };

            Ok(Json(AuthResponse {
                token,
                user: user_profile,
            }))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/login", data = "<request>")]
pub async fn login(
    mut db: DbPool,
    request: Json<LoginRequest>,
) -> Result<Json<AuthResponse>, Status> {
    // Find user by username
    let user_result = sqlx::query_as!(
        User,
        "SELECT id, username, email, password_hash, created_at, updated_at FROM users WHERE username = $1",
        request.username
    )
    .fetch_optional(&mut **db)
    .await;

    let user = match user_result {
        Ok(Some(user)) => user,
        Ok(None) => return Err(Status::Unauthorized),
        Err(_) => return Err(Status::InternalServerError),
    };

    // Verify password
    match verify_password(&request.password, &user.password_hash) {
        Ok(true) => {}
        Ok(false) => return Err(Status::Unauthorized),
        Err(_) => return Err(Status::InternalServerError),
    }

    // Create JWT token
    let token = match create_jwt_token(&user) {
        Ok(token) => token,
        Err(_) => return Err(Status::InternalServerError),
    };

    let user_profile = UserProfile::from(user);

    Ok(Json(AuthResponse {
        token,
        user: user_profile,
    }))
}

#[get("/profile")]
pub async fn get_profile(
    mut db: DbPool,
    auth_user: AuthenticatedUser,
) -> Result<Json<UserProfile>, Status> {
    let user_result = sqlx::query_as!(
        UserProfile,
        "SELECT id, username, email, created_at FROM users WHERE id = $1",
        auth_user.user_id
    )
    .fetch_optional(&mut **db)
    .await;

    match user_result {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/profile")]
pub async fn delete_account(
    mut db: DbPool,
    auth_user: AuthenticatedUser,
) -> Result<Status, Status> {
    // Delete user account (cascade will handle teams, team_members, packages, package_tags)
    let result = sqlx::query!("DELETE FROM users WHERE id = $1", auth_user.user_id)
        .execute(&mut **db)
        .await;

    match result {
        Ok(rows) if rows.rows_affected() > 0 => Ok(Status::NoContent),
        Ok(_) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/users/<username>")]
pub async fn get_user_by_username(
    mut db: DbPool,
    username: &str,
) -> Result<Json<UserProfile>, Status> {
    let user_result = sqlx::query_as!(
        UserProfile,
        "SELECT id, username, email, created_at FROM users WHERE username = $1",
        username
    )
    .fetch_optional(&mut **db)
    .await;

    match user_result {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}
