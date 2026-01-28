mod routes;
mod septa;
mod sfw;

use axum::Router;
use std::net::SocketAddr;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "is_septa_fcked=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting SEPTA status tracker...");

    // Boot SEPTA data fetchers
    septa::rr::boot().await;
    septa::bus::boot().await;

    // Give the fetchers a moment to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Set up CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build our application with routes
    let app = Router::new()
        .merge(routes::create_routes())
        .nest_service("/public", ServeDir::new("public"))
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // Get port from environment or default to 5000
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "5000".to_string())
        .parse::<u16>()
        .unwrap_or(5000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
