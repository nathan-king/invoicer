use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Client {
    pub id: i64,
    pub name: String,
    pub email: Option<String>,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct CreateClient {
    pub name: String,
    pub email: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateClient {
    pub name: String,
    pub email: Option<String>,
}
