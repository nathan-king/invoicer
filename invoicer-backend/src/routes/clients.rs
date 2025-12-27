use crate::handlers;
use axum::{
    Router,
    routing::{delete, get, post, put},
};
use sqlx::SqlitePool;

pub fn client_routes() -> Router<SqlitePool> {
    Router::new()
        .route("/clients", post(handlers::create_client))
        .route("/clients", get(handlers::list_clients))
        .route("/clients/{id}", get(handlers::get_client))
        .route("/clients/{id}", put(handlers::update_client))
        .route("/clients/{id}", delete(handlers::delete_client))
}
