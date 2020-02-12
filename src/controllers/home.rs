extern crate redis_async;

use actix_web::{HttpResponse, Error, web, get};
use actix_redis::{RedisActor, Command, RespValue};
use actix::prelude::*;
use redis_async::*;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct UpdateParams {
    id: i32,
}

pub async fn index(redis: web::Data<Addr<RedisActor>>) -> Result<HttpResponse, Error> {
//    redis.send(Command()).await;

    let value = format!("quannaaa");
    println!("{}", value);
    let res = resp_array!["SET", "mydomain", "value"];
    let res222 = redis.send(Command(res)).await;
    Ok(HttpResponse::Ok().body("Hello index"))
}

pub async fn update(params: web::Path<UpdateParams>) -> Result<HttpResponse, Error> {
    let redis_uri: &str = &"127.0.0.1:6379";//&dotenv::var("REDIS_URI").unwrap().as_str();
    let redis_executor = RedisActor::start(redis_uri);
    let value = format!("{:?}", params.id);
    let res = resp_array!["SET", "update", value];
    redis_executor.send(Command(res)).await;
    Ok(HttpResponse::Ok().body("Hello update"))
}

pub async fn hello() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hello world"))
}