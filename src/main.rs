use actix_web::{App, HttpServer, middleware};

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
    dotenv().ok();
    let redis_uri: &str = &dotenv::var("REDIS_URI").unwrap();
    let redis_client = RedisConnectionManager::new(redis::parse_redis_url(redis_uri).unwrap()).unwrap();
    let pool = r2d2::Pool::builder().build(redis_client).unwrap();
    //Initialize App Server
    HttpServer::new( move || {
        App::new()
            // add redis connection pool
            .data(pool.clone())
            // config routers from home routers
            .configure(routers::home::init)
    }).bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await
}