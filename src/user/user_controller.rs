use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::PgPool;
use super::{user_service, user_model::*};
use crate::errors::CustomError;

#[get("/users")]
async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
    let users = user_service::get_users(pool).await;

    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            let error = CustomError::from(e);
            HttpResponse::InternalServerError().json(error.to_string())
        } 
    }
}

#[get("/user/orders/{id}")]
async fn get_user_orders(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    let orders = user_service::get_user_orders(pool, id).await;

    match orders {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(e) => {
            let error = CustomError::from(e);
            HttpResponse::InternalServerError().json(error.to_string())
        }
    }
}

#[post("/user")]
async fn create_user(pool: web::Data<PgPool>, user_dto: web::Json<CreateUserDTO>,) -> impl Responder {
    let user = user_service::create_user(pool, user_dto).await;

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            let error = CustomError::from(e);
            HttpResponse::InternalServerError().json(error.to_string())
        }
    }
}

#[put("/user/{id}")]
async fn update_user(pool: web::Data<PgPool>, user_dto: web::Json<UpdateUserDTO>, id: web::Path<i32>) -> impl Responder {
    
    let user = user_service::update_user(pool, user_dto, id).await;

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            let error = CustomError::from(e);
            HttpResponse::InternalServerError().json(error.to_string())
        }
    }
}

#[delete("/user/{id}")]
async fn delete_user(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    let user = user_service::delete_user(pool, id).await;

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            let error = CustomError::from(e);
            HttpResponse::InternalServerError().json(error.to_string())
        }
    }
}