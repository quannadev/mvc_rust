use actix::prelude::*;
use actix_web::{Error, HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};

use crate::services::redis_service;
use crate::actors::redis_actor::RedisActor;

type RedisAddr = web::Data<Addr<RedisActor>>;

#[derive(Deserialize, Serialize)]
pub struct Response {
    status: i32,
    message: String,
}

pub async fn index() -> impl Responder {
    let res = Response {
        status: 200,
        message: format!("hello api"),
    };
    HttpResponse::Ok().json(res)
}

#[derive(Deserialize)]
pub struct UserParams {
    id: i32,
}

pub async fn set_user(params: web::Json<UserParams>, redis: RedisAddr) -> Result<HttpResponse, Error> {
    let uid = params.id;
    Ok(HttpResponse::Ok().body("value"))
}

pub async fn get_user(params: web::Path<UserParams>, redis: RedisAddr) -> impl Responder {
    HttpResponse::Ok().body("OK")
}
