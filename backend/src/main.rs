use axum::{
    extract::Extension,
    routing::{get, post},
    Json, Router,
};
use sqlx::PgPool;
use std::net::SocketAddr;
use tokio::sync::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to the database");

    let app = Router::new()
        .route("/", get(root))
        .route("/login", post(login))
        .route("/art", post(create_art))
        .layer(Extension(Arc::new(Mutex::new(pool))));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, CREA platform!"
}

async fn login() -> &'static str {
    // Implement OAuth 2.0 login logic here
    "Login endpoint"
}

async fn create_art() -> &'static str {
    // Implement ART creation logic here
    "Create ART endpoint"
}
