use crate::handlers::invoices::{create_invoice, list_invoices};
use axum::{
    Router,
    routing::{get, post},
};
use sqlx::SqlitePool;

pub fn invoice_routes() -> Router<SqlitePool> {
    Router::new()
        .route("/invoices", post(create_invoice))
        .route("/invoices", get(list_invoices))
}
