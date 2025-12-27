use crate::models::{Client, CreateClient};
use axum::{Json, extract::State};
use sqlx::SqlitePool;

pub async fn create_client(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateClient>,
) -> Json<Client> {
    let record = sqlx::query!(
        r#"
        INSERT INTO clients (name, email, created_at)
        VALUES (?, ?, datetime('now'))
        RETURNING id, name, email, created_at
        "#,
        payload.name,
        payload.email
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to insert client");

    Json(Client {
        id: record.id,
        name: record.name,
        email: record.email,
        created_at: record.created_at,
    })
}
