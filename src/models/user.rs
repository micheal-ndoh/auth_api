use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub password_hash: String, // Hashed in production
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub enum Role {
    Admin,
    User,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterResponse {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema, Default)]
pub struct ProfileUpdateRequest {
    #[validate(length(min = 2, max = 50))]
    pub firstname: Option<String>,
    #[validate(length(min = 2, max = 50))]
    pub lastname: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = 6))]
    pub password: Option<String>,
    // Profile picture as a URL or base64 string (optional)
    pub profile_picture: Option<String>,
}
