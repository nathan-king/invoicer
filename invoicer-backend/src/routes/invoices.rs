use crate::handlers::invoices::create_invoice;
use axum::{Router, routing::post};
use sqlx::SqlitePool;

pub fn invoice_routes() -> Router<SqlitePool> {
    Router::new().route("/invoices", post(create_invoice))
}
