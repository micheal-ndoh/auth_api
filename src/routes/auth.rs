use axum::extract::{Extension, State};
use axum::{http::StatusCode, response::IntoResponse, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use regex;
use regex::Regex;
use serde_json::json;
use sqlx::prelude::*;
use std::sync::Arc;
use utoipa::OpenApi;
use validator::Validate;

use crate::middleware::auth::Claims;
use crate::models::user::{ProfileUpdateRequest, RegisterRequest, User};
use crate::models::{LoginRequest, LoginResponse, Role};
use crate::AppState;

#[derive(OpenApi)]
#[openapi(paths(login), components(schemas(LoginRequest, LoginResponse)))]
pub struct AuthApi;

#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials")
    )
)]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(&state.db)
        .await
        .unwrap();

    if let Some(user) = user {
        if verify(&payload.password, &user.password_hash).unwrap() {
            let claims = Claims {
                sub: user.email.clone(),
                role: user.role.clone(),
                exp: (chrono::Utc::now() + chrono::Duration::minutes(10)).timestamp() as usize,
            };
            let config = state.config.clone();
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(config.jwt_secret.as_ref()),
            )
            .unwrap();
            return (StatusCode::OK, Json(LoginResponse { token })).into_response();
        }
    }
    (
        StatusCode::UNAUTHORIZED,
        Json(json!({"error": "Invalid credentials"})),
    )
        .into_response()
}

#[utoipa::path(
    post,
    path = "/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully"),
        (status = 400, description = "Bad request")
    )
)]
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    if payload.email.is_empty() || payload.password.as_deref().unwrap_or("").is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "email and password are required"})),
        )
            .into_response();
    }
    let email_regex = regex::Regex::new(r"^[\w\.-]+@[\w\.-]+\.[a-zA-Z]{2,}$").unwrap();
    if !email_regex.is_match(&payload.email) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid email format"})),
        )
            .into_response();
    }
    if payload
        .firstname
        .chars()
        .filter(|c| c.is_alphabetic())
        .count()
        < 2
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "First name must contain at least 2 letters"})),
        )
            .into_response();
    }
    if payload
        .lastname
        .chars()
        .filter(|c| c.is_alphabetic())
        .count()
        < 2
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Last name must contain at least 2 letters"})),
        )
            .into_response();
    }
    let hashed_password = hash(payload.password.as_deref().unwrap_or(""), DEFAULT_COST).unwrap();
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (email, firstname, lastname, password_hash, role) VALUES ($1, $2, $3, $4, $5) RETURNING id, email, firstname, lastname, password_hash, role"
    )
    .bind(&payload.email)
    .bind(&payload.firstname)
    .bind(&payload.lastname)
    .bind(&hashed_password)
    .bind("User")
    .fetch_one(&state.db)
    .await;
    match user {
        Ok(_user) => (
            StatusCode::CREATED,
            Json(json!({"message": "User registered successfully"})),
        )
            .into_response(),
        Err(e) => (
            StatusCode::CONFLICT,
            Json(json!({"error": format!("Failed to register user: {}", e)})),
        )
            .into_response(),
    }
}

#[utoipa::path(
    patch,
    path = "/profile",
    request_body = ProfileUpdateRequest,
    responses(
        (status = 200, description = "Profile updated successfully", body = User),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized")
    ),
    security(("api_key" = []))
)]
pub async fn update_profile(
    State(state): State<AppState>,
    Extension(user): Extension<Arc<User>>,
    Json(payload): Json<ProfileUpdateRequest>,
) -> impl IntoResponse {
    // Log the incoming payload for debugging
    eprintln!("PATCH /profile payload: {:?}", payload);

    // Validate request
    if let Err(e) = payload.validate() {
        eprintln!("Validation error: {}", e);
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": format!("Validation error: {}", e)})),
        )
            .into_response();
    }

    // Prepare fields for update
    let mut hashed_password: Option<String> = None;
    if let Some(ref pass) = payload.password {
        match hash(pass, DEFAULT_COST) {
            Ok(h) => hashed_password = Some(h),
            Err(e) => {
                eprintln!("Password hash error: {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Failed to hash password"})),
                )
                    .into_response();
            }
        }
    }

    // Update user in DB using COALESCE for partial update
    let updated = sqlx::query_as::<_, User>(
        "UPDATE users SET \
            firstname = COALESCE($1, firstname),\
            lastname = COALESCE($2, lastname),\
            email = COALESCE($3, email),\
            password_hash = COALESCE($4, password_hash)\
         WHERE id = $5\
         RETURNING id, email, firstname, lastname, password_hash, role",
    )
    .bind(payload.firstname.as_ref())
    .bind(payload.lastname.as_ref())
    .bind(payload.email.as_ref())
    .bind(hashed_password.as_ref())
    .bind(user.id)
    .fetch_one(&state.db)
    .await;
    match updated {
        Ok(updated_user) => (StatusCode::OK, Json(updated_user)).into_response(),
        Err(e) => {
            eprintln!("Failed to update profile for user {}: {}", user.id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to update profile: {}", e)})),
            )
                .into_response()
        }
    }
}
