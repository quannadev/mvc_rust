extern crate dotenv;
extern crate log;
extern crate r2d2_redis;

use actix_web::{App, HttpServer, web, HttpResponse};
use actix_web::middleware::Logger;
use r2d2_redis::{RedisConnectionManager, r2d2, redis};
use r2d2::Pool;
use dotenv::dotenv;
use std::env;

mod routers;
mod controllers;
mod services;
mod utils;
mod models;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //config logger
    env::set_var("RUST_LOG", "actix_web=warn");
    env_logger::init();
    dotenv().ok();

    //config redis
    let redis_uri: &str = &dotenv::var("REDIS_URI").unwrap();
    let redis_executor = RedisConnectionManager::new(redis_uri).unwrap();
    let redis_manger = Pool::builder().build(redis_executor).unwrap();
    let redis_clone = redis_manger.clone();
    //Initialize App Server
    // move is necessary to give closure below ownership of redis_executor
    println!("Starting App Server...");
    HttpServer::new(move || {
        App::new()
            // add redis connection pool
            .data(redis_clone.clone())
            //enable logger
            .wrap(Logger::new("%a - %t - %s"))
            // config routers from home routers
//            .configure(routers::home::init)
            .configure(routers::api::init)
            .route("/", web::get().to(|| HttpResponse::Ok().body("home")))
            .default_service(web::route().to(|| HttpResponse::Unauthorized().body("You are lost!")))
    }).bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await
}
