use serde::Deserialize;

#[derive(Deserialize)]
pub struct Order {
    pub id: String,
    pub paid: i64,
    pub currency: String,
}

#[derive(Deserialize)]
pub struct Customer {
    pub email: String,
    pub phone: Option<i64>,
}

#[derive(Deserialize)]
pub struct ModificationParams {
    pub amount: i64,
    //TODO: Add authentication
}

#[derive(Deserialize)]
pub struct RewardParams {
    pub amount: f64,
    //TODO: Add authentication
}

#[derive(Deserialize)]
pub struct CustomerOrder {
    pub order: Order,
    pub customer: Customer,
    pub reward_params: Option<RewardParams>,
}
