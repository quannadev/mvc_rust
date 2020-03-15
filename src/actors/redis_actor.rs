use r2d2_redis::r2d2::{Pool};
use r2d2_redis::RedisConnectionManager;
use actix::{Actor, SyncContext};

pub struct RedisActor {
    pub client: Pool<RedisConnectionManager>
}
impl RedisActor {
    pub fn new(client: Pool<RedisConnectionManager>) -> Self {
        RedisActor {client}
    }
}
impl Actor for RedisActor {
    type Context = SyncContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("RedisActor started")
    }
}

