use axum::{Json, Router, routing::get};
use serde::Serialize;
use sqlx::sqlite::SqlitePoolOptions;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let database_url = "sqlite:data/invoice.db";

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to connect to database");

    let app = Router::new()
        .route("/health", get(health))
        .merge(routes::clients::client_routes())
        .merge(routes::invoices::invoice_routes())
        .layer(CorsLayer::permissive())
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::info!("Server running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}
