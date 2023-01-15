use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct Customer {
    pub id: Option<u16>,
    #[validate(length(min = 3))]
    pub name: String,
    #[validate(range(min = 0))]
    pub balance: Option<f64>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct CustomerEdit {
    #[validate(length(min = 3))]
    pub name: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct Transfer {
    pub id_from: u16,
    pub id_to: u16,
    #[validate(range(min = 1))]
    pub amount: f64,
}

#[derive(Serialize)]
pub struct APIResponse {
    pub message: String,
}
