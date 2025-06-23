use std::sync::{Arc, Mutex};

use axum::http::HeaderValue;
use axum::http::Method;
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod middleware;
pub mod models;
pub mod routes;
pub mod utils;
use dotenvy::dotenv;

use crate::{
    middleware::auth::auth_middleware,
    routes::{auth, protected},
};

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Arc<utils::Config>,
    pub db: PgPool,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to DB");

    #[derive(OpenApi)]
    #[openapi(
        info(
            title = "Auth API",
            description = "A simple auth API",
            license(name = "MIT", identifier = "MIT")
        ),
        paths(
            auth::login,
            auth::register,
            protected::admin_route,
            protected::user_route
        ),
        components(schemas(
            models::User,
            models::Role,
            models::LoginRequest,
            models::LoginResponse
        ))
    )]
    struct ApiDoc;

    let state = AppState {
        config: Arc::new(utils::load_env()),
        db,
    };

    let cors = CorsLayer::new()
        .allow_origin(
            "https://auth-api-lemon.vercel.app"
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    let app = Router::new()
        .route("/admin", get(protected::admin_route))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/login", post(auth::login))
        .route("/register", post(auth::register))
        .layer(cors)
        .with_state(state);
    println!("Running on http://localhost:3001/swagger-ui");

    println!("\nRunning on http://localhost:3000/api-docs/openapi.json");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// curl -X GET "http://localhost:3000/admin" \
//   -H "accept: application/json" \
//   -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJtaWNAZ21haWwuY29tIiwicm9sZSI6IlVzZXIiLCJleHAiOjE3NTA2NzkxODR9.DuNQMTyzjzfZ2wu70JaorVuWxe3V7iymcXxqomz8UBs"

// curl -X 'POST' \
//   'http://localhost:3000/register' \
//   -H 'accept: application/json' \
//   -H 'Content-Type: application/json' \
//   -d '{
//   "password": "password",
//   "confirm_password": "password",
//   "username": "admin"
// }'
// PGPASSWORD=CrOqatkjSybFHxvYSFHpJFkQPkcxOjMS psql -h turntable.proxy.rlwy.net -U postgres -p 52084 -d railway
