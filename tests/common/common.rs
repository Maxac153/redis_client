use r2d2_redis::{redis, RedisConnectionManager};
use redis::Commands;

pub struct TestSetup {
    redis_url: String,
}

pub fn load_test_params() -> (String, String) {
    let host_redis = std::env::var("TEST_HOST").unwrap_or("0.0.0.0".to_string());
    let port_redis = std::env::var("TEST_PORT").unwrap_or("6379".to_string());
    (host_redis, port_redis)
}

impl TestSetup {
    pub fn new(redis_host: String, redis_port: String) -> Self {
        Self {
            redis_url: format!("redis://{}:{}/", redis_host, redis_port),
        }
    }

    pub fn setup_test_list_data(&self, key: &str, data: Vec<&str>) {
        let mut con: redis::Connection = redis::Client::open(self.redis_url.clone())
            .unwrap()
            .get_connection()
            .unwrap();

        for value in data {
            con.lpush(key, value).unwrap()
        }
    }

    pub fn setup_test_hash_data(&self, key: &str, field: &str, data: &str) {
        let mut con: redis::Connection = redis::Client::open(self.redis_url.clone())
            .unwrap()
            .get_connection()
            .unwrap();
        con.hset::<&str, &str, &str, ()>(key, field, data).unwrap();
    }

    pub fn drop(&self) {
        let mut con: redis::Connection = redis::Client::open(self.redis_url.clone())
            .unwrap()
            .get_connection()
            .unwrap();
        let keys: Vec<String> = con.scan().unwrap().collect();
        for key in keys {
            con.del::<&str, ()>(&key).unwrap();
        }
    }

    pub fn init_pool(&self) -> r2d2::Pool<RedisConnectionManager> {
        let manager = RedisConnectionManager::new(self.redis_url.clone())
            .expect("Failed to create Redis connection manager");
        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool")
    }
}
