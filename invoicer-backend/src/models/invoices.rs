use serde::{Deserialize, Serialize};

enum InvoiceStatus {
    Draft,
    Sent,
    Paid,
    Cancelled,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Invoice {
    pub id: i64,
    pub client_id: i64,
    pub status: InvoiceStatus,
    pub issued_at: String,
    pub due_at: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateInvoice {
    pub client_id: i64,
    pub due_at: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateInvoice {
    pub status: InvoiceStatus,
    pub due_at: Option<String>,
}
