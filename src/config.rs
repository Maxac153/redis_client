use std::env;

#[derive(Clone)]
pub struct Config {
    redis_host: String,
    redis_port: String,
    redis_pool_connection: u32,
    workers: usize,
    playload_limit: usize,
    request_timeout_sec: u64,
}

fn get_env_var(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

impl Config {
    pub fn new() -> Config {
        Config {
            redis_host: get_env_var("REDIS_HOST", "0.0.0.0"),
            redis_port: get_env_var("REDIS_PORT", "6379"),
            redis_pool_connection: get_env_var("REDIS_POOL_CONNECTION", "10")
                .parse::<u32>()
                .unwrap_or(10),
            workers: get_env_var("WORKERS", "4").parse::<usize>().unwrap_or(4),
            playload_limit: get_env_var("PLAYLOAD_LIMIT", "1")
                .parse::<usize>()
                .unwrap_or(1),
            request_timeout_sec: get_env_var("REQUEST_TIMEOUT_SEC", "60")
                .parse::<u64>()
                .unwrap_or(60),
        }
    }

    pub fn get_redis_host(&self) -> &str {
        &self.redis_host
    }

    pub fn get_redis_port(&self) -> &str {
        &self.redis_port
    }

    pub fn get_redis_pool_connection(&self) -> u32 {
        self.redis_pool_connection
    }

    pub fn get_workers(&self) -> usize {
        self.workers
    }

    pub fn get_playload_limit(&self) -> usize {
        self.playload_limit
    }

    pub fn get_request_timeout_sec(&self) -> u64 {
        self.request_timeout_sec
    }
}
