use crate::models::{Client, CreateClient, UpdateClient};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
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

pub async fn get_client(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<Json<Client>, StatusCode> {
    let row = sqlx::query(
        r#"
        SELECT id, name, email, created_at
        FROM clients
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match row {
        Some(row) => Ok(Json(Client {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            created_at: row.get("created_at"),
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn update_client(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Json(payload): Json<UpdateClient>,
) -> Result<Json<Client>, StatusCode> {
    let row = sqlx::query(
        r#"
        UPDATE clients
        SET name = ?, email = ?
        WHERE id = ?
        RETURNING id, name, email, created_at
        "#,
    )
    .bind(payload.name)
    .bind(payload.email)
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match row {
        Some(row) => Ok(Json(Client {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            created_at: row.get("created_at"),
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}
