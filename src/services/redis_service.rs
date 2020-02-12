extern crate r2d2_redis;

use r2d2_redis::{redis, RedisConnectionManager, r2d2};
use r2d2_redis::redis::{RedisError, RedisResult, FromRedisValue, PipelineCommands, Connection, from_redis_value, Value};
use std::ops::DerefMut;
use r2d2_redis::r2d2::PooledConnection;
use std::borrow::Borrow;

pub async fn hset(mut db: PooledConnection<RedisConnectionManager>, key: String, field: String, value: String) -> Result<String, RedisError> {
    let mut pipe = redis::pipe();
    pipe.hset(key, field, value).execute(db.deref_mut());
    Ok("OK".to_string())
}
pub async fn hget(mut db: PooledConnection<RedisConnectionManager>, key: String, field: String) -> Result<String, RedisError> {
    let mut pipe = redis::pipe();
    let data = pipe.hget(key, field).query::<Vec<String>>(db.deref_mut()).unwrap();
    Ok(data.concat())
}