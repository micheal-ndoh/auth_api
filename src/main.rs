use std::sync::{Arc, Mutex};

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
pub mod utils;
use dotenvy::dotenv;

use crate::{
    middleware::auth::auth_middleware,
    routes::{auth, protected},
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Arc<utils::Config>,
    pub users: Arc<Mutex<Vec<models::User>>>,
}

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
        info(title = "Auth API", description = "A simple auth API"),
        paths(auth::login, auth::register, protected::admin_route),
        components(schemas(
            models::User,
            models::Role,
            models::LoginRequest,
            models::LoginResponse
        ))
    )]
    struct ApiDoc;

    let state = AppState {
        users: Arc::new(Mutex::new(vec![])),
        config: Arc::new(utils::load_env()),
    };

    let app = Router::new()
        .route("/admin", get(protected::admin_route))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/login", post(auth::login))
        .route("/register", post(auth::register))
        .layer(CorsLayer::permissive())
        .with_state(state);
    println!("Running on http://localhost:3000/swagger-ui");

    println!("\nRunning on http://localhost:3000/api-docs/openapi.json");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// curl -X 'GET'                                                                                                                                                              [17:27:06]
//   'http://localhost:3000/admin'
//   -H 'accept: application/json'
//   -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJtaWNoZWFsQGdtYWlsLmNvbSIsInJvbGUiOiJVc2VyIiwiZXhwIjoxNzUwNzQ3NDE2fQ.4Y3Gmh2uWSAj4ixkuBaahyLP91qsI4xGTATqQzH4RD4'

// curl -X 'POST' \
//   'http://localhost:3000/register' \
//   -H 'accept: application/json' \
//   -H 'Content-Type: application/json' \
//   -d '{
//   "password": "password",
//   "confirm_password": "password",
//   "username": "admin"
// }'
