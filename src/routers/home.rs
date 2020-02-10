use actix_web::web;
use crate::controllers::*;
pub fn init(cfg: &mut web::ServiceConfig) {
    web::resource("/").route(web::get().to(home::index));
}