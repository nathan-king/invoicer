use crate::handlers::invoices::{create_invoice, get_invoice_by_id, list_invoices};
use axum::{
    Router,
    routing::{get, post},
};
use sqlx::SqlitePool;

pub fn invoice_routes() -> Router<SqlitePool> {
    Router::new()
        .route("/invoices", post(create_invoice))
        .route("/invoices", get(list_invoices))
        .route("/invoices/{id}", get(get_invoice_by_id))
}
