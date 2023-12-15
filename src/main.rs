use axum::http;
use axum::routing::{get, Router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(health));

    // run our app with hyper, listening globally on port 8000
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health() -> http::StatusCode {
    http::StatusCode::OK
}