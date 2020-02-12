extern crate dotenv;
extern crate log;

use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_cors::Cors;
use log::{trace, info};
use actix_redis::{RedisActor, RedisSession};
use dotenv::dotenv;

mod routers;
mod controllers;
mod services;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //config logger
    std::env::set_var("RUST_LOG", "actix_web=warn");
    env_logger::init();
    //config redis
    dotenv().ok();
    info!("Config redis connection...");
    let redis_uri: &str = &"127.0.0.1:6379";//&dotenv::var("REDIS_URI").unwrap().as_str();
    let redis_executor= RedisActor::start(redis_uri);

    trace!("Start http server...");
    //Initialize App Server
    // move is necessary to give closure below ownership of redis_executor
    HttpServer::new( move || {
        App::new()
            // add redis connection pool
//            .data(redis_executor.clone())
            //enable logger
//            .wrap(Logger::default())
            .wrap(Logger::new("%a - %t - %s"))
            //enable Cors configure
//            .wrap(Cors::default())
//            .wrap(RedisSession::new(redis_uri, &[0;32]))
            // config routers from home routers
            .configure(routers::home::init)
    }).bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await
}
