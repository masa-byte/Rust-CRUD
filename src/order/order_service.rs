use actix_web::web;
use sqlx::PgPool;
use super::order_model::*;
use crate::errors::CustomError;

pub async fn get_orders(pool: web::Data<PgPool>) -> Result<Vec<Order>, CustomError>{
    let orders = sqlx::query_as!(
        Order,
        "SELECT id, customer_id, order_date, status AS \"status: OrderStatus\", total_price FROM orders"
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(orders)
}

pub async fn get_order(pool: web::Data<PgPool>, id: web::Path<i32>) -> Result<OrderWithItems, CustomError> {
    let order = sqlx::query_as!(
        Order,
        "SELECT id, customer_id, order_date, status AS \"status: OrderStatus\", total_price FROM orders WHERE id = $1",
        id.as_ref(),
    )
    .fetch_one(pool.get_ref())
    .await?;

    let order_items = sqlx::query_as!(
        OrderItem,
        "SELECT order_id, product_id, quantity FROM order_items WHERE order_id = $1",
        id.into_inner(),
    )
    .fetch_all(pool.get_ref())
    .await?;

    let order_with_items = OrderWithItems {
        order: order,
        items: order_items,
    };

    Ok(order_with_items)
}

pub async fn create_order(pool: web::Data<PgPool>, order_dto: web::Json<CreateOrderDTO>) -> Result<Order, CustomError> {

    let mut total_price = 0;
    for item in &order_dto.items {
        let product = sqlx::query!(
            "SELECT price FROM products WHERE id = $1",
            item.product_id,
        )
        .fetch_one(pool.get_ref())
        .await?;

        total_price += product.price * item.quantity;
    }

    let order = sqlx::query_as!(
        Order,
        "INSERT INTO orders (customer_id, total_price) VALUES ($1, $2) RETURNING id, customer_id, order_date, status AS \"status: OrderStatus\", total_price",
        order_dto.customer_id,
        total_price,
    )
    .fetch_one(pool.get_ref())
    .await?;

    for item in &order_dto.items {
        sqlx::query!(
            "INSERT INTO order_items (order_id, product_id, quantity) VALUES ($1, $2, $3)",
            order.id,
            item.product_id,
            item.quantity,
        )
        .execute(pool.get_ref())
        .await?;
    }

    Ok(order)
}

pub async fn update_order(pool: web::Data<PgPool>, order_dto: web::Json<UpdateOrderDTO>, id: web::Path<i32>) -> Result<Order, CustomError> {
    let order = sqlx::query_as!(
        Order,
        "UPDATE orders SET status = COALESCE($1, status) WHERE id = $2 RETURNING id, customer_id, order_date, status AS \"status: OrderStatus\", total_price",
        &order_dto.status as &OrderStatus,
        id.into_inner(),
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(order)
}

pub async fn delete_order(pool: web::Data<PgPool>, id: web::Path<i32>) -> Result<&'static str, CustomError>{
    let _ = sqlx::query!(
        "DELETE FROM orders WHERE id = $1 RETURNING id",
        id.into_inner(),
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok("Successfully deleted order")
}