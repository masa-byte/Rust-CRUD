use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String
}

#[derive(Debug, Deserialize)]
pub struct CreateUserDTO {
    pub name: String,
    pub email: String
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserDTO {
    pub name: Option<String>,
    pub email: Option<String>
}
