use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct OrderSchema{
    pub id: String,
    pub paid: usize,
    pub currency: String,
    pub customer_email: String,
    pub percentage: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerSchema{
    pub email: String,
    pub phone: Option<String>,
    pub points: i64,
}
