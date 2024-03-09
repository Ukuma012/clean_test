use actix_session::storage::RedisActorSessionStore;

pub struct RedisConnection {}

impl RedisConnection {
    pub fn new() -> RedisActorSessionStore {
        let redis_url = dotenvy::var("REDIS_URL").expect("REDIS_URL must be set");

        RedisActorSessionStore::new(redis_url)
    }
}
