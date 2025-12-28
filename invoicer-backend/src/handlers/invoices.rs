use crate::models::invoices::{
    CreateInvoice, Invoice, InvoiceQuery, InvoiceStatus, InvoiceWithClientName,
};
use axum::{
    Json,
    extract::{Path, Query, State},
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

fn order_by_from_query(sort: Option<&str>) -> &'static str {
    match sort {
        Some("issued_at") => "invoices.issued_at DESC",
        Some("due_at") => "invoices.due_at DESC",
        Some("status") => "invoices.status DESC",
        Some("id") => "invoices.id DESC",
        _ => "invoices.id DESC",
    }
}

pub async fn list_invoices(
    State(pool): State<SqlitePool>,
    Query(query): Query<InvoiceQuery>,
) -> Json<Vec<InvoiceWithClientName>> {
    let order_by = order_by_from_query(query.sort.as_deref());

    Json(list_invoices_ordered_by(&pool, order_by).await)
}

async fn list_invoices_ordered_by(pool: &SqlitePool, order_by: &str) -> Vec<InvoiceWithClientName> {
    let query = format!(
        r#"
        SELECT
            invoices.id,
            invoices.client_id,
            clients.name AS client_name,
            invoices.status,
            invoices.issued_at,
            invoices.due_at
        FROM invoices
        JOIN clients ON clients.id = invoices.client_id
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

            InvoiceWithClientName {
                id: row.get("id"),
                client_id: row.get("client_id"),
                client_name: row.get("client_name"),
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
) -> Result<Json<InvoiceWithClientName>, StatusCode> {
    let row = sqlx::query(
        r#"
        SELECT
            invoices.id,
            invoices.client_id,
            clients.name AS client_name,
            invoices.status,
            invoices.issued_at,
            invoices.due_at
        FROM invoices
        JOIN clients ON clients.id = invoices.client_id
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

            Ok(Json(InvoiceWithClientName {
                id: row.get("id"),
                client_id: row.get("client_id"),
                client_name: row.get("client_name"),
                status: InvoiceStatus::from_str(&status_str),
                issued_at: row.get("issued_at"),
                due_at: row.get("due_at"),
            }))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}
