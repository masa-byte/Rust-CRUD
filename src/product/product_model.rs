
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

type Euro = i32;

#[derive(Debug, Serialize, FromRow)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: Euro
}

#[derive(Debug, Deserialize)]
pub struct CreateProductDTO {
    pub name: String,
    pub description: String,
    pub price: Euro
}

#[derive(Debug, Deserialize)]
pub struct UpdateProductDTO {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<Euro>
}