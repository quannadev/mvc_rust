extern crate dotenv;
extern crate log;
extern crate r2d2_redis;

use std::env;

use actix::prelude::*;
use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use r2d2::Pool;
use r2d2_redis::{r2d2, redis, RedisConnectionManager};
use r2d2_redis::redis::parse_redis_url;

use crate::services::redis_exec::RedisExecutor;

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
    let redis_uri: String = env::var("REDIS_URI").expect("Redis URI must be set");
    let redis_executor = RedisConnectionManager::new(parse_redis_url(&redis_uri).unwrap()).unwrap();
    let redis_manger = Pool::builder().build(redis_executor).unwrap();
    let redis_executor_addr = SyncArbiter::start(7, move || {
        RedisExecutor::new(redis_manger.clone())
    });
    //Initialize App Server
    // move is necessary to give closure below ownership of redis_executor
    println!("Starting App Server...");
    HttpServer::new(move || {
        App::new()
            // add redis connection pool
            .data(redis_executor_addr.clone())
            //enable logger
            .wrap(Logger::new("%a - %t - %s"))
            // config routers from home routers
//            .configure(routers::home::init)
            .configure(routers::api::init)
            .route("/", web::get().to(|| HttpResponse::Ok().body("home")))
            .default_service(web::route().to(|| HttpResponse::Unauthorized()))
    }).bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await
}
