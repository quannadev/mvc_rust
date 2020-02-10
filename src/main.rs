use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use ::actix::prelude::*;
use log::trace;

extern crate r2d2_redis;
extern crate dotenv;

use dotenv::dotenv;
use r2d2_redis::{r2d2, redis, RedisConnectionManager};

mod routers;
mod controllers;
mod services;

use services::redis::RedisExecutor;
use services::redis_exec;

const REDIS_WORKERS: usize = 7;
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //config logger
    pretty_env_logger::init();
    //config redis
    dotenv().ok();
    trace!("Config redis connection...");
    let redis_uri: &str = &dotenv::var("REDIS_URI").unwrap();
    let redis_client = RedisConnectionManager::new(redis::parse_redis_url(redis_uri).unwrap()).unwrap();
    let pool = r2d2::Pool::builder().build(redis_client).unwrap();
    let redis_executor = SyncArbiter::start(REDIS_WORKERS, move || {
        RedisExecutor::new(pool.clone())
    });
    trace!("Start http server...");
    //Initialize App Server
//    let addr = ([127, 0, 0, 1], 8080).into();
    HttpServer::new( move || {
        App::new()
            // add redis connection pool
            .data(redis_executor.clone())
            .wrap(Logger::default())
            // config routers from home routers
            .configure(routers::home::init)
    }).bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await
}
