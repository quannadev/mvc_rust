extern crate r2d2_redis;
extern crate serde_json;

use actix::prelude::*;
use failure::Error;
use r2d2_redis::redis::Commands;

use crate::services::redis_exec::RedisExecutor;


///SET Expire value
/// @key: String
/// @value: StringF
pub struct RedisSETEX {
    pub key: String,
    pub value: String,
    pub exp: usize,
}

impl Message for RedisSETEX {
    type Result = String;
}

impl Handler<RedisSETEX> for RedisExecutor {
    type Result = String;
    fn handle(&mut self, msg: RedisSETEX, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.pool.get().unwrap();
        let RedisSETEX { key, value, exp } = msg;
        conn.set_ex::<String, String, ()>(key, value, exp).unwrap();
        String::from("OK")
    }
}

///SET redis value is vec string
/// @key: string
/// @value: Vec<String>
/// result: String
pub struct WriteJSON {
    key: String,
    value: Vec<String>,
}

impl Message for WriteJSON {
    type Result = Result<(), Error>;
}

impl Handler<WriteJSON> for RedisExecutor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: WriteJSON, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.pool.get().unwrap();
        let result = conn.set(msg.key, serde_json::to_string(&msg.value).unwrap()).unwrap();
        Ok(result)
    }
}


///GET redis Vec value from key
/// key: String
/// result: Vec<String>
pub struct ReadVecResult {
    key: String,
}

impl Message for ReadVecResult {
    type Result = Result<Vec<String>, Error>;
}

impl Handler<ReadVecResult> for RedisExecutor {
    type Result = Result<Vec<String>, Error>;

    fn handle(&mut self, msg: ReadVecResult, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.pool.get().unwrap();
        let result: Option<String> = conn.get(msg.key).unwrap();
        if result.is_none() {
            return Ok(vec![]);
        }
        Ok(serde_json::from_str(&result.unwrap()).unwrap())
    }
}

/// GET value from key
/// @key: String
/// @result: String
pub struct RedisGet(pub String);

impl Message for RedisGet {
    type Result = String;
}

impl Handler<RedisGet> for RedisExecutor {
    type Result = String;
    fn handle(&mut self, msg: RedisGet, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.pool.get().unwrap();
        let result: Option<String> = conn.get(msg.0).unwrap();
        result.unwrap_or_else(|| "null".to_string())
    }
}