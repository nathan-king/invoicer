use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Client {
    pub id: i64,
    pub name: String,
    pub email: Option<String>,
    pub created_at: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateClient {
    pub name: String,
    pub email: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateClient {
    pub name: String,
    pub email: Option<String>,
}
