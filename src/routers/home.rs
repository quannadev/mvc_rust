//use actix_web::web;
//
//use crate::controllers::home;
//use crate::controllers::api_controller as controllers;
//pub fn init(cfg: &mut web::ServiceConfig) {
//    cfg.service(
//        web::scope("/")
//            .route("", web::get().to(home::hello))
//            .route("/home", web::get().to(home::index))
//    );
//    cfg.service(
//        web::scope("/api/v1/")
//            .route("", web::get().to(controllers::index))
//            .route("/{id}", web::get().to(controllers::get_user))
//    );
//}
