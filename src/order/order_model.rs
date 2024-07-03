use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, FromRow)]
pub struct Order {
    pub id: i32,
    pub customer_id: i32,
    pub order_date: NaiveDateTime,
    pub status: OrderStatus,
    pub total_price: i32
}

#[derive(Debug, Deserialize, FromRow)]
pub struct CreateOrderDTO {
    pub customer_id: i32,
    pub items: Vec<CreateOrderItemDTO>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct OrderItem {
    pub order_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderItemDTO {
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrderDTO {
    pub status: OrderStatus,
}

#[derive(Debug, Serialize)]
pub struct OrderWithItems {
    pub order: Order,
    pub items: Vec<OrderItem>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "order_status")] 
pub enum OrderStatus {
    Created,
    Shipped,
    Completed,
    Cancelled,
}