use std::env;

use actix::{Addr, SyncArbiter};
use r2d2_redis::redis::parse_redis_url;
use r2d2_redis::RedisConnectionManager;

use crate::actors::redis_actor::RedisActor;
use r2d2_redis::r2d2::Pool;

pub mod redis_actor;

pub fn init_redis(number_actor: usize) -> Addr<RedisActor> {
    let redis_uri = env::var("REDIS_URI")
        .unwrap_or("redis://localhost:6379".to_string());
    let url = parse_redis_url(&redis_uri).unwrap();
    let connection = RedisConnectionManager::new(url);
    let conn = match connection {
        Ok(conn) => conn,
        Err(err) => {
            panic!("init_redis error {}", err.to_string());
        }
    };
    let redis_pool = Pool::builder().build(conn).unwrap();
    SyncArbiter::start(number_actor, move || {
        RedisActor::new(redis_pool.clone())
    })
}
