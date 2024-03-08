use actix_session::storage::RedisSessionStore;

pub struct RedisConnection {}

impl RedisConnection {
    pub async fn new() -> Result<RedisSessionStore, std::io::Error> {
        let redis_url = dotenvy::var("REDIS_URL").expect("REDIS_URL must be set");

        match RedisSessionStore::new(redis_url).await {
            Ok(redis_store) => Ok(redis_store),
            Err(err) => Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to connect Redis: {}", err))),
        }
    }
}
