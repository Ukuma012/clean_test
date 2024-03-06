use actix_redis::RedisSession;
use actix_session::CookieSession;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use std::sync::Arc;

pub struct RedisConnection {}

impl RedisConnection {
    pub fn new() -> Arc<RedisSession> {
        let redis_url = dotenvy::var("REDIS_URL").expect("REDIS_URL must be set");
        let mut csp_rng = ChaCha20Rng::from_entropy();
        let mut redis_data = [0u8; 32];
        csp_rng.fill_bytes(&mut redis_data);

        Arc::new(RedisSession::new(&redis_url, &redis_data))
    }
}
