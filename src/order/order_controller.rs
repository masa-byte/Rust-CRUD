use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::PgPool;
use super::{order_service, order_model::*};
use crate::errors::CustomError;

#[get("/orders")]
async fn get_orders(pool: web::Data<PgPool>) -> impl Responder {
    let orders = order_service::get_orders(pool).await;

    match orders {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(e) => {
            let error = CustomError::from(e);
            HttpResponse::InternalServerError().json(error.to_string())
        }
    }
}

#[get("/order/{id}")]
async fn get_order(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    let order_with_items = order_service::get_order(pool, id).await;

    match order_with_items {
        Ok(order_with_items) => HttpResponse::Ok().json(order_with_items),
        Err(e) => {
            let error = CustomError::from(e);
            HttpResponse::InternalServerError().json(error.to_string())
        }
    }
}

#[post("/order")]
async fn create_order(pool: web::Data<PgPool>, order_dto: web::Json<CreateOrderDTO>) -> impl Responder {
    let order = order_service::create_order(pool, order_dto).await;

    match order {
        Ok(order) => HttpResponse::Ok().json(order),
        Err(e) => {
            let error = CustomError::from(e);
            HttpResponse::InternalServerError().json(error.to_string())
        }
    }
}

#[put("/order/{id}")]
async fn update_order(pool: web::Data<PgPool>, order_dto: web::Json<UpdateOrderDTO>, id: web::Path<i32>) -> impl Responder {
    let order = order_service::update_order(pool, order_dto, id).await;

    match order {
        Ok(order) => HttpResponse::Ok().json(order),
        Err(e) => {
            let error = CustomError::from(e);
            HttpResponse::InternalServerError().json(error.to_string())
        }
    }
}

#[delete("/order/{id}")]
async fn delete_order(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    let order = order_service::delete_order(pool, id).await;

    match order {
        Ok(order) => HttpResponse::Ok().json(order),
        Err(e) => {
            let error = CustomError::from(e);
            HttpResponse::InternalServerError().json(error.to_string())
        }
    }
}