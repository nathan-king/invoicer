use crate::models::invoices::{CreateInvoice, Invoice, InvoiceStatus};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::{Row, SqlitePool};

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

pub async fn list_invoices_by_id(State(pool): State<SqlitePool>) -> Json<Vec<Invoice>> {
    Json(list_invoices_ordered_by(&pool, "id DESC").await)
}

async fn list_invoices_ordered_by(pool: &SqlitePool, order_by: &str) -> Vec<Invoice> {
    let query = format!(
        r#"
        SELECT id, client_id, status, issued_at, due_at
        FROM invoices
        ORDER BY {}
        "#,
        order_by
    );

    let rows = sqlx::query(&query)
        .fetch_all(pool)
        .await
        .expect("Failed to fetch invoices");

    rows.into_iter()
        .map(|row| {
            let status_str: String = row.get("status");

            Invoice {
                id: row.get("id"),
                client_id: row.get("client_id"),
                status: InvoiceStatus::from_str(&status_str),
                issued_at: row.get("issued_at"),
                due_at: row.get("due_at"),
            }
        })
        .collect()
}

pub async fn get_invoice_by_id(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<Json<Invoice>, StatusCode> {
    let row = sqlx::query(
        r#"
        SELECT id, client_id, status, issued_at, due_at
        FROM invoices
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match row {
        Some(row) => {
            let status_str: String = row.get("status");

            Ok(Json(Invoice {
                id: row.get("id"),
                client_id: row.get("client_id"),
                status: InvoiceStatus::from_str(&status_str),
                issued_at: row.get("issued_at"),
                due_at: row.get("due_at"),
            }))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}
