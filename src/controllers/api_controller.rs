extern crate serde_json;

use actix::prelude::*;
use actix_web::{Error, HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};

use crate::models::user_model;
use crate::services::redis_service;
use crate::services::redis_exec::RedisExecutor;

type RedisActor = web::Data<Addr<RedisExecutor>>;

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

pub async fn set_user(params: web::Json<UserParams>, redis: RedisActor) -> Result<HttpResponse, Error> {
    let uid = params.id;
    let rest = redis.send(redis_service::RedisSETEX{
        key: String::from("quanna"),
        value: String::from(uid.to_string()),
        exp: 10
    }).await;
    Ok(HttpResponse::Ok().body("Ok"))
}

pub async fn get_user(params: web::Path<UserParams>, redis: RedisActor) -> impl Responder {

    let user_id = params.id;
    let result = redis.send(redis_service::RedisGet("quanna".to_string(),)).await;
    HttpResponse::Ok().body(result.unwrap())
}
