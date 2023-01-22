use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct Customer {
    pub id: Option<u16>,
    #[validate(length(min = 3))]
    pub name: String,
    #[validate(range(min = 0))]
    pub balance: Option<f64>,
    pub created_at: Option<String>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct CustomerEdit {
    #[validate(length(min = 3))]
    pub name: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct Token {
    pub text: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct Claims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
    pub role: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct Payment {
    pub id: Option<u16>,
    pub created_at: Option<String>,
    #[serde(rename = "customerId")]
    pub customer_id: Option<u16>,
    pub amount: f64,
    #[serde(rename = "receiverCode")]
    pub receiver_code: String,
    pub reference: String,
    pub note: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct Transfer {
    pub id: Option<u16>,
    #[serde(rename = "idFrom")]
    pub id_from: u16,
    #[serde(rename = "idTo")]
    pub id_to: u16,
    #[validate(range(min = 1))]
    pub amount: f64,
    pub created_at: Option<String>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct TransferHuman {
    pub id: u16,
    pub name_from: String,
    pub name_to: String,
    pub amount: f64,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct APIResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct Money {
    #[validate(range(min = 1))]
    pub amount: f64,
}
