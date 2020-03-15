use actix::{Handler, Message};
use r2d2_redis::redis::Commands;

use crate::actors::redis_actor::RedisActor;

#[warn(unused_must_use)]
#[allow(dead_code)]
pub enum RedisKeys {
    SetInc
}

impl RedisKeys {
    pub fn to_string(&self) -> String {
        match self {
            RedisKeys::SetInc => "set_incr".to_string()
        }
    }
}


pub struct RGet {
    pub key: RedisKeys
}

impl Message for RGet {
    type Result = Result<String, ()>;
}

impl Handler<RGet> for RedisActor {
    type Result = Result<String, ()>;

    fn handle(&mut self, msg: RGet, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.client.get().unwrap();
        match conn.get::<String, String>(msg.key.to_string()) {
            Ok(value) => {
                Ok(value)
            }
            _ => Err(())
        }
    }
}

pub struct RDel {
    pub key: RedisKeys
}

impl Message for RDel {
    type Result = Result<String, ()>;
}

impl Handler<RDel> for RedisActor {
    type Result = Result<String, ()>;

    fn handle(&mut self, msg: RDel, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.client.get().unwrap();
        match conn.del::<String, String>(msg.key.to_string()) {
            Ok(value) => Ok(value),
            _ => Err(())
        }
    }
}

