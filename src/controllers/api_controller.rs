extern crate serde_json;
extern crate r2d2_redis;

use actix_web::{HttpResponse, web, Responder};
use serde::{Deserialize, Serialize};
use crate::utils::responder_template;
use crate::models::user_model;
use r2d2_redis::{RedisConnectionManager, r2d2, redis};
use redis::Commands;
use rand::prelude::*;
use r2d2_redis::redis::PipelineCommands;
use crate::services::redis_service;

type RedisExecutor = web::Data<r2d2::Pool<RedisConnectionManager>>;

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
    responder_template::data(serde_json::to_value(&res).unwrap())
}

#[derive(Deserialize)]
pub struct UserParams {
    id: i32,
}

pub async fn set_user(params: web::Json<UserParams>, db: RedisExecutor) -> impl Responder {
    let uid = params.id;
    let conn = db.get().unwrap();
//    let _ = redis_service::hset( conn, "users".to_string(), uid.to_string(), "quanna".to_string()).await;
    let data = redis_service::hget( conn, "users".to_string(), uid.to_string()).await;
    HttpResponse::Ok().body(format!("{:?}", data.unwrap()))
}

pub async fn get_user(params: web::Path<UserParams>, db: RedisExecutor) -> impl Responder {
    let mut conn = db.get().unwrap();

    let user_id = params.id;
    println!("Redis server is open");
//    let mut pipe = redis::pipe();
//    let data = pipe.hget("user", user_id).execute(conn.deref_mut());
//    let uuid: u32 = FromStr::from_str(data).unwrap();
    let user = user_model::User {
        id: 123123,
        name: String::from("quanna"),
        email: String::from("quanna@example.com"),
    };
    HttpResponse::Ok().json(user)
}