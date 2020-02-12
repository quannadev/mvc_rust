use actix_web::web;

use crate::controllers::*;

pub fn init(_cfg: &mut web::ServiceConfig) {
    _cfg.service(web::resource("/").route(web::get().to(home::hello)));
    _cfg.service(web::resource("/home").route(web::get().to(home::index)));
    _cfg.service(web::resource("/post/{id}").route(web::get().to(home::update)));
}
