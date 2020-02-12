extern crate r2d2_redis;

use std::ops::DerefMut;

use actix::prelude::*;
use actix_web::web;
use failure::Error;
use r2d2_redis::redis::{Commands, RedisError, pipe, PipelineCommands};
use serde::{Deserialize, Serialize};

use crate::services::redis_exec::RedisExecutor;

type RedisConnection = web::Data<r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>>;
#[derive(Deserialize, Serialize)]
pub struct HSETData {
    pub key: String,
    pub field: String,
    pub value: String,
}

impl HSETData {
    fn new(key: String, field: String, value: String) -> HSETData {
        HSETData {
            key,
            field,
            value,
        }
    }
}


pub async fn hset(db: RedisConnection, data: HSETData) -> Result<bool, RedisError> {
    let mut conn = db.get().unwrap();
    let mut pipe = pipe();
    let pi = pipe.hset(data.key, data.field, data.value).execute(conn.deref_mut());
    Ok(true)
}

pub async fn hget(mut db: RedisConnection, key: String, field: String) -> Result<String, RedisError> {
    let mut conn = db.get().unwrap();
    let mut pipe = pipe();
    let data = pipe.hget(key, field).query::<Vec<String>>(conn.deref_mut()).unwrap();
    Ok(data.concat())
}
