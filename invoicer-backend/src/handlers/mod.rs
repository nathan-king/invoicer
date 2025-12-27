use crate::models::{Client, CreateClient};
use axum::{Json, extract::State};
use sqlx::{Row, SqlitePool};

pub async fn create_client(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateClient>,
) -> Json<Client> {
    let row = sqlx::query(
        r#"
        INSERT INTO clients (name, email, created_at)
        VALUES (?, ?, datetime('now'))
        RETURNING id, name, email, created_at
        "#,
    )
    .bind(payload.name)
    .bind(payload.email)
    .fetch_one(&pool)
    .await
    .expect("Failed to insert client");

    Json(Client {
        id: row.get("id"),
        name: row.get("name"),
        email: row.get("email"),
        created_at: row.get("created_at"),
    })
}

pub async fn list_clients(
    State(pool): State<SqlitePool>,
) -> axum::Json<Vec<crate::models::Client>> {
    let rows = sqlx::query(
        r#"
        SELECT id, name, email, created_at
        FROM clients
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch clients");

    let clients = rows
        .into_iter()
        .map(|row| crate::models::Client {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            created_at: row.get("created_at"),
        })
        .collect();

    Json(clients)
}
