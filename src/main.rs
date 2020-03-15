extern crate dotenv;
#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_json;

use std::env;

use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::middleware::Logger;
use dotenv::dotenv;

mod routers;
mod controllers;
mod services;
mod utils;
mod models;
mod actors;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //config logger
    env::set_var("RUST_MVC_LOG", "actix_web=info");
    env_logger::builder().format_module_path(false).init();
    dotenv().ok();
    info!("Starting App Server...");
    let server = HttpServer::new(move || {
        App::new()
            .data(actors::init_redis(1))
            .wrap(Logger::new("%a - %t - %s"))
            .configure(routers::api::init)
            .route("/", web::get().to(|| HttpResponse::Ok().body("home")))
            .default_service(web::route().to(|| HttpResponse::Unauthorized()))
    });
    server.workers(1)
        .bind("0.0.0.0:8080")
        .unwrap()
        .run()
        .await
}
