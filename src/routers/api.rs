use actix_web::web;

use crate::controllers::api_controller as controllers;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/")
            .route("", web::get().to(controllers::index))
            .route("/users", web::post().to(controllers::set_user))
            .route("/users/{id}", web::get().to(controllers::get_user))
    );
}