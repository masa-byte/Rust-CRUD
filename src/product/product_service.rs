use actix_web::web;
use sqlx::PgPool;
use super::product_model::*;
use crate::errors::CustomError;

pub async fn get_products(pool: web::Data<PgPool>) -> Result<Vec<Product>, CustomError> {
    let products = sqlx::query_as!(
        Product,
        "SELECT id, name, description, price FROM products"
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(products)
}

pub async fn create_product(pool: web::Data<PgPool>, product_dto: web::Json<CreateProductDTO>,) -> Result<Product, CustomError> {
    let product = sqlx::query_as!(
        Product,
        "INSERT INTO products (name, description, price) VALUES ($1, $2, $3) RETURNING id, name, description, price",
        product_dto.name,
        product_dto.description,
        product_dto.price,
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(product)
}

pub async fn update_product(pool: web::Data<PgPool>, product_dto: web::Json<UpdateProductDTO>, id: web::Path<i32>) -> Result<Product, CustomError> {
    
    let product = sqlx::query_as!(
        Product,
        "UPDATE products SET name = COALESCE($1, name), description = COALESCE($2, description), price = COALESCE($3, price) WHERE id = $4 RETURNING id, name, description, price",
        product_dto.name,
        product_dto.description,
        product_dto.price,
        id.into_inner(),
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(product)
}

pub async fn delete_product(pool: web::Data<PgPool>, id: web::Path<i32>) -> Result<&'static str , CustomError> {
    let _ = sqlx::query!(
        "DELETE FROM products WHERE id = $1 RETURNING id",
        id.into_inner(),
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok("Product deleted successfully")
}


