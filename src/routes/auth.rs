use axum::{http::StatusCode, response::IntoResponse, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use std::env;
use utoipa::OpenApi;

use crate::middleware::auth::Claims;
use crate::models::{LoginRequest, LoginResponse, Role, User};

#[derive(OpenApi)]
#[openapi(paths(login), components(schemas(LoginRequest, LoginResponse)))]
pub struct AuthApi;

// RegistrationRequest struct for user registration payload
#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct RegistrationRequest {
    pub username: String,
    pub password: String,
    pub confirm_password: String,
}

// Registration endpoint for new users. Assigns role User.
#[utoipa::path(
    post,
    path = "/register",
    request_body = RegistrationRequest,
    responses(
        (status = 201, description = "Registration successful", body = User),
        (status = 400, description = "Bad request")
    )
)]
pub async fn register(Json(payload): Json<RegistrationRequest>) -> impl IntoResponse {
    // Ensure password and confirm_password match
    if payload.password != payload.confirm_password {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Passwords do not match"})),
        )
            .into_response();
    }
    // In production, check if user exists and hash password
    let user = User {
        id: 2,
        username: payload.username,
        password: payload.password, // Should hash in production
        role: Role::User,           // Assign User role to new registrations
    };
    (StatusCode::CREATED, Json(user)).into_response()
}

#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials")
    )
)]
pub async fn login(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    // In production, verify against a database
    if payload.username == "admin" && payload.password == "password" {
        let claims = Claims {
            sub: payload.username.clone(),
            role: Role::Admin,
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        };

        // Load JWT secret from environment variable for decoding
        let key_str = match env::var("JWT_SECRET") {
            Ok(val) => val,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "JWT secret not set"})),
                )
                    .into_response();
            }
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(key_str.as_ref()),
        )
        .unwrap();

        return (StatusCode::OK, Json(LoginResponse { token })).into_response();
    }

    (
        StatusCode::UNAUTHORIZED,
        Json(json!({"error": "Invalid credentials"})),
    )
        .into_response()
}
