use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

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
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterResponse {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}
