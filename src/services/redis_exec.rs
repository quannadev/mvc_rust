use r2d2_redis::redis::{Commands, Connection};

pub trait Client: Commands {
//    fn get(&self, key: &str) -> RedisResult<()> {
//        self.get(key)
//    }
//    fn set(&self, key: &str, value: &str) {
//        self.set(key, value)
//    }
}

impl Client for Connection {}