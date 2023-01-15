use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Customer {
    pub id: u16,
    pub name: String,
    pub balance: f64,
}

#[derive(Serialize)]
pub struct APIResponse {
    pub message: String,
}

