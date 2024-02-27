use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::book::OrderType;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaceOrderArgs {
    pub order_type: OrderType,
    pub price: u32,
    pub quantity: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CancelOrderArgs {
    pub order_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    PlaceOrder(PlaceOrderArgs),
    CancelOrder(CancelOrderArgs),
    ViewL2Book,
}