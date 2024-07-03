use actix_web::web;
use sqlx::PgPool;
use super::user_model::*;
use crate::errors::CustomError;
use crate::order::order_model::*;

pub async fn get_users(pool: web::Data<PgPool>) -> Result<Vec<User>, CustomError>{
    let users = sqlx::query_as!(
        User,
        "SELECT id, name, email FROM customers"
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(users)
}

pub async fn get_user_orders(pool: web::Data<PgPool>, id: web::Path<i32>) -> Result<Vec<Order>, CustomError>{
    let orders = sqlx::query_as!(
        Order,
        "SELECT id, customer_id, order_date, status AS \"status: OrderStatus\", total_price FROM orders WHERE customer_id = $1",
        id.into_inner(),
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(orders)
}

pub async fn create_user(pool: web::Data<PgPool>, user_dto: web::Json<CreateUserDTO>,) -> Result<User, CustomError>{
    let user = sqlx::query_as!(
        User,
        "INSERT INTO customers (name, email) VALUES ($1, $2) RETURNING id, name, email",
        user_dto.name,
        user_dto.email,
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(user)
}

pub async fn update_user(pool: web::Data<PgPool>, user_dto: web::Json<UpdateUserDTO>, id: web::Path<i32>) -> Result<User, CustomError> {
    
    let user = sqlx::query_as!(
        User,
        "UPDATE customers SET name = COALESCE($1, name), email = COALESCE($2, email) WHERE id = $3 RETURNING id, name, email",
        user_dto.name,
        user_dto.email,
        id.into_inner(),
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(user)
}

pub async fn delete_user(pool: web::Data<PgPool>, id: web::Path<i32>) -> Result<&'static str, CustomError> {
    let _ = sqlx::query_as!(
        User,
        "DELETE FROM customers WHERE id = $1 RETURNING id, name, email",
        id.into_inner(),
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok("User deleted successfully")
}