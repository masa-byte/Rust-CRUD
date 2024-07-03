use actix_web::{App, HttpServer, web};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
// use sqlx::migrate;
use dotenv::dotenv;

mod user;
mod product;
mod order;
mod errors;

use user::*;
use product::*;
use order::*;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = establish_connection().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(user_controller::get_users)
            .service(user_controller::create_user)
            .service(user_controller::update_user)
            .service(user_controller::delete_user)
            .service(user_controller::get_user_orders)

            .service(product_controller::get_products)
            .service(product_controller::create_product)
            .service(product_controller::update_product)
            .service(product_controller::delete_product)

            .service(order_controller::get_orders)
            .service(order_controller::get_order)
            .service(order_controller::create_order)
            .service(order_controller::update_order)
            .service(order_controller::delete_order)
        })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn establish_connection() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}