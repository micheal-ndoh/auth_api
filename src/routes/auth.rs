use axum::extract::State;
use axum::{http::StatusCode, response::IntoResponse, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use sqlx::prelude::*;
use utoipa::OpenApi;

use crate::middleware::auth::Claims;
use crate::models::user::{RegisterRequest, User};
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
    if payload.email.is_empty() || payload.password.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "email and password are required"})),
        )
            .into_response();
    }
    let hashed_password = hash(&payload.password, DEFAULT_COST).unwrap();
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (email, firstname, lastname, password_hash, role) VALUES ($1, $2, $3, $4, $5) RETURNING id, email, firstname, lastname, password_hash, role"
    )
    .bind(&payload.email)
    .bind(&payload.firstname)
    .bind(&payload.lastname)
    .bind(&hashed_password)
    .bind(Role::User)
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
