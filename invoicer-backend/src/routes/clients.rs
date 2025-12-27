use crate::handlers::clients::{
    create_client, delete_client, get_client, list_clients, update_client,
};
use axum::{
    Router,
    routing::{delete, get, post, put},
};
use sqlx::SqlitePool;

pub fn client_routes() -> Router<SqlitePool> {
    Router::new()
        .route("/clients", post(create_client))
        .route("/clients", get(list_clients))
        .route("/clients/{id}", get(get_client))
        .route("/clients/{id}", put(update_client))
        .route("/clients/{id}", delete(delete_client))
}
