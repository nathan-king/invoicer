use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceStatus {
    Draft,
    Sent,
    Paid,
    Cancelled,
}

impl InvoiceStatus {
    pub fn from_str(value: &str) -> Self {
        match value {
            "draft" => InvoiceStatus::Draft,
            "sent" => InvoiceStatus::Sent,
            "paid" => InvoiceStatus::Paid,
            "cancelled" => InvoiceStatus::Cancelled,
            _ => InvoiceStatus::Draft,
        }
    }
}

impl InvoiceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            InvoiceStatus::Draft => "draft",
            InvoiceStatus::Sent => "sent",
            InvoiceStatus::Paid => "paid",
            InvoiceStatus::Cancelled => "cancelled",
        }
    }
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct InvoiceWithClientName {
    pub id: i64,
    pub client_id: i64,
    pub client_name: String,
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
pub struct InvoiceQuery {
    pub sort: Option<String>,
}

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "snake_case")]
// pub struct UpdateInvoice {
//     pub status: InvoiceStatus,
//     pub due_at: Option<String>,
// }
