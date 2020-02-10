use actix_web::web;

use crate::controllers::*;

pub fn init(_cfg: &mut web::ServiceConfig) {
    _cfg.route("/", web::get().to(home::index));
}
