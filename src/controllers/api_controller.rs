extern crate serde_json;

use actix::prelude::*;
use actix_web::{Error, HttpResponse, Responder, web};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::user_model;
use crate::services::redis_service;

type RedisActor = web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>;

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
    let data_redis = redis_service::HSETData {
        key: String::from("quanna"),
        field: String::from("123123"),
        value: String::from("aaaa")
    };
    let _ = redis_service::hset(redis, data_redis).await;
    Ok(HttpResponse::Ok().body("Ok"))
}

pub async fn get_user(params: web::Path<UserParams>, db: RedisActor) -> impl Responder {
    let mut conn = db.get().unwrap();

    let user_id = params.id;
    println!("Redis server is open");
    let user = user_model::User {
        id: 123123,
        name: String::from("quanna"),
        email: String::from("quanna@example.com"),
    };
    HttpResponse::Ok().json(user)
}
