use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::PgPool;
use super::{product_service, product_model::*};
use crate::errors::CustomError;

#[get("/products")]
async fn get_products(pool: web::Data<PgPool>) -> impl Responder {
    let products = product_service::get_products(pool).await;

    match products {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(e) => {
            let error = CustomError::from(e);
            HttpResponse::InternalServerError().json(error.to_string())
        }
    }
}

#[post("/product")]
async fn create_product(pool: web::Data<PgPool>, product_dto: web::Json<CreateProductDTO>,) -> impl Responder {
    let product = product_service::create_product(pool, product_dto).await;

    match product {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(e) => {
            let error = CustomError::from(e);
            HttpResponse::InternalServerError().json(error.to_string())
        }
    }
}

#[put("/product/{id}")]
async fn update_product(pool: web::Data<PgPool>, product_dto: web::Json<UpdateProductDTO>, id: web::Path<i32>) -> impl Responder {
    let product = product_service::update_product(pool, product_dto, id).await;

    match product {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(e) => {
            let error = CustomError::from(e);
            HttpResponse::InternalServerError().json(error.to_string())
        }
    }
}

#[delete("/product/{id}")]
async fn delete_product(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    let product = product_service::delete_product(pool, id).await;

    match product {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(e) => {
            let error = CustomError::from(e);
            HttpResponse::InternalServerError().json(error.to_string())
        }
    }
}