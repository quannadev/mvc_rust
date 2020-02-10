use actix_web::{App, HttpServer, middleware};
use actix_web::middleware::session::SessionStorage;
use actix_redis::RedisSessionBackend;

extern crate r2d2_redis;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use r2d2_redis::{r2d2, redis, RedisConnectionManager};

mod routers;
mod controllers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //config redis
    let redis_uri: &str = env::var("REDIS_URI").unwrap_or_else("redis://localhost:6379").as_str();
    let redis_client = RedisConnectionManager::new(redis_uri).unwrap();
    let pool = r2d2::Pool::builder().build(redis_client).unwrap();
    //Initialize App Server
    HttpServer::new(move || {
        App::new()
            // enable logger
            .middleware(middleware::Logger::default())
            // cookie session middleware
            .middleware(SessionStorage::new(
                RedisSessionBackend::new(redis_uri, &[0; 32])
            ))
            // add redis connection pool
            .data(pool.clone())
            // config routers from home routers
            .configure(routers::home::init)
    }).bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await
}