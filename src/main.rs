use std::time::Duration;

use actix_files::Files;
use actix_multipart::form::MultipartFormConfig;
use actix_web::{web, App, HttpServer};
use redis_client::routes::{
    init_routes_pages::init_routes_pages, init_routes_redis::init_routes_redis,
};
use tera::Tera;

use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;

mod config;

const MB: usize = 1048576;
const GB: usize = 1073741824;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config: config::Config = config::Config::new();
    let redis_url: String = format!("{}:{}", config.get_redis_host(), config.get_redis_port());
    let manager = RedisConnectionManager::new(format!("redis://{}/", redis_url)).unwrap();

    let redis_pool_connection: Pool<RedisConnectionManager> = Pool::builder()
        .max_size(config.get_redis_pool_connection())
        .test_on_check_out(false)
        .build(manager)
        .unwrap();

    let multipart_total_limit = config.get_multipart_total_limit();
    let multipart_mermory_limit = config.get_multipart_mermory_limit();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(
                MultipartFormConfig::default()
                    .total_limit(multipart_total_limit * GB)
                    .memory_limit(multipart_mermory_limit * MB),
            ))
            .app_data(web::Data::new(redis_url.to_string()))
            .app_data(web::Data::new(redis_pool_connection.clone()))
            .app_data(web::Data::new(Tera::new("templates/*").unwrap()))
            .service(Files::new("/static", "static"))
            .configure(init_routes_pages)
            .configure(init_routes_redis)
    })
    .bind("0.0.0.0:8080")?
    .client_request_timeout(Duration::from_secs(config.get_request_timeout_sec()))
    .workers(config.get_workers())
    .run()
    .await
}
