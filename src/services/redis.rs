extern crate r2d2_redis;
use ::actix::prelude::*;
use r2d2_redis::{redis,RedisConnectionManager};
use r2d2_redis::r2d2::Pool;

pub struct RedisExecutor {
    pub pool: Pool<RedisConnectionManager>
}

impl RedisExecutor {
    pub fn new(pool: Pool<RedisConnectionManager>) -> Self {
        RedisExecutor { pool }
    }
}
impl Actor for RedisExecutor {
    type Context = SyncContext<Self>;
}
