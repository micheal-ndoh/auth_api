use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
pub mod middleware;
pub mod models;
pub mod routes;
use crate::middleware::auth::auth_middleware;
use crate::routes::{auth, protected};
use dotenv;
mod utils;

use std::sync::{Arc, Mutex};

// AppState holds all registered users in memory
#[derive(Debug, Clone)]
pub struct AppState {
    pub users: Vec<models::User>,
}

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
        info(title = "Auth API", description = "A simple auth API"),
        paths(
            auth::login,
            protected::admin_route,
            protected::user_route,
            auth::register
        ),
        components(schemas(
            models::User,
            models::Role,
            models::LoginRequest,
            models::LoginResponse
        ))
    )]
    struct ApiDoc;

    dotenv::dotenv().ok(); // Load environment variables from .env file
                           // Create a thread-safe, shared AppState using Arc<Mutex<...>>
    let state = Arc::new(Mutex::new(AppState { users: vec![] }));

    // Pass the same state to all routes so registration and login share users
    let app = Router::new()
        .route("/admin", get(protected::admin_route))
        .layer(axum::middleware::from_fn(auth_middleware))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/login", post(auth::login))
        .route("/register", post(auth::register))
        .route("/user", get(protected::user_route))
        .layer(CorsLayer::permissive())
        .with_state(state);

    println!("Server running on http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// curl -X 'GET'                                                                                                                                                              [17:27:06]
//   'http://localhost:3000/admin'
//   -H 'accept: application/json'
//   -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhZG1pbiIsInJvbGUiOiJBZG1pbiIsImV4cCI6MTc1MDQzNjk5Mn0.BvFjNJ46EV93tg-we8iFeBaU83XrnnXpLEAL1Mplp64'

// curl -X 'POST' \
//   'http://localhost:3000/register' \
//   -H 'accept: application/json' \
//   -H 'Content-Type: application/json' \
//   -d '{
//   "password": "password",
//   "confirm_password": "password",
//   "username": "admin"
// }'
