use axum::extract::State;
use axum::{http::StatusCode, response::IntoResponse, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use std::env;
use std::sync::{Arc, Mutex};
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

// Registration endpoint for new users. Assigns role User and stores in shared AppState.
#[utoipa::path(
    post,
    path = "/register",
    request_body = RegistrationRequest,
    responses(
        (status = 201, description = "Registration successful", body = User),
        (status = 400, description = "Bad request")
    )
)]
pub async fn register(
    State(state): State<Arc<Mutex<crate::AppState>>>, // Shared, thread-safe state
    Json(payload): Json<RegistrationRequest>,
) -> impl IntoResponse {
    if payload.password != payload.confirm_password {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Passwords do not match"})),
        )
            .into_response();
    }
    // Hash the password using bcrypt for security
    let hashed = hash(&payload.password, DEFAULT_COST).unwrap();

    // Lock the state to get mutable access to users
    let mut app_state = state.lock().unwrap();
    let new_user = User {
        id: app_state.users.len() as i32 + 1, // Simple ID generator
        username: payload.username.clone(),
        password: hashed,
        role: Role::User,
    };
    app_state.users.push(new_user.clone()); // Store user in shared state
    (StatusCode::CREATED, Json(new_user)).into_response()
}

// Login endpoint checks credentials against users in shared AppState
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
    State(state): State<Arc<Mutex<crate::AppState>>>, // Shared, thread-safe state
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    // Lock the state to access users
    let app_state = state.lock().unwrap();
    if let Some(user) = app_state
        .users
        .iter()
        .find(|u| u.username == payload.username)
    {
        // Verify password using bcrypt
        if bcrypt::verify(&payload.password, &user.password).unwrap() {
            let claims = Claims {
                sub: user.username.clone(),
                role: user.role.clone(),
                exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
            };
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
    }
    (
        StatusCode::UNAUTHORIZED,
        Json(json!({"error": "Invalid credentials"})),
    )
        .into_response()
}
