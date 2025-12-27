use axum::{Json, extract::State};
use sqlx::{Row, SqlitePool};

use crate::models::invoices::{CreateInvoice, Invoice, InvoiceStatus};

pub async fn create_invoice(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateInvoice>,
) -> Json<Invoice> {
    let row = sqlx::query(
        r#"
        INSERT INTO invoices (client_id, status, issued_at, due_at)
        VALUES (?, ?, datetime('now'), ?)
        RETURNING id, client_id, status, issued_at, due_at
        "#,
    )
    .bind(payload.client_id)
    .bind(InvoiceStatus::Draft.as_str())
    .bind(payload.due_at)
    .fetch_one(&pool)
    .await
    .expect("Failed to insert invoice");

    let status_str: String = row.get("status");

    Json(Invoice {
        id: row.get("id"),
        client_id: row.get("client_id"),
        status: InvoiceStatus::from_str(&status_str),
        issued_at: row.get("issued_at"),
        due_at: row.get("due_at"),
    })
}
