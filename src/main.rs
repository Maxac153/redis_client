use actix_files::Files;
use actix_web::{web, App, HttpServer};
use actix_web_prom::PrometheusMetrics;
use actix_web_prom::PrometheusMetricsBuilder;
use prometheus::opts;
use prometheus::IntCounterVec;
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use redis_client::handlers::redis::{
    add, change_ttl, download_dump_key, read, rename_key, reset, status, upload_dump_key,
};
use redis_client::{
    models::{response::Response, status::StatusJson, status_key::StatusKey},
    routes::{init_routes_pages::init_routes_pages, init_routes_redis::init_routes_redis},
};
use std::time::Duration;
use tera::Tera;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod config;

const GB: usize = 1073741824;

// Swagger
#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "Redis Client", description = "Redis Client management endpoints.")
    ),
    paths(
        add::add_list,
        add::add_hash,
        change_ttl::change_ttl,
        download_dump_key::download_dump_key,
        download_dump_key::download_dump_all_keys,
        read::read_list,
        read::read_hash,
        rename_key::rename_key,
        reset::reset_key,
        reset::reset_all_keys,
        status::status_json,
        status::status_key,
        upload_dump_key::upload_dump_key,
        upload_dump_key::upload_dump_all_keys
    ),
    components(
        schemas(
            Response,
            StatusKey,
            StatusJson
        ),
    ),
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config: config::Config = config::Config::new();
    let redis_url: String = format!("{}:{}", config.get_redis_host(), config.get_redis_port());
    let manager = RedisConnectionManager::new(format!("redis://{}/", redis_url)).unwrap();

    // Redis pool
    let redis_pool_connection: Pool<RedisConnectionManager> = Pool::builder()
        .max_size(config.get_redis_pool_connection())
        .test_on_check_out(false)
        .build(manager)
        .unwrap();

    let playload_limit = config.get_playload_limit();
    HttpServer::new(move || {
        // Prometheus
        let prometheus: PrometheusMetrics = PrometheusMetricsBuilder::new("api")
            .endpoint("/metrics")
            .build()
            .unwrap();

        let counter_opts = opts!("counter", "some random counter").namespace("api");
        let counter = IntCounterVec::new(counter_opts, &["endpoint", "method", "status"]).unwrap();

        App::new()
            .wrap(prometheus)
            .app_data(web::PayloadConfig::new(playload_limit * GB))
            .app_data(web::Data::new(counter))
            .app_data(web::Data::new(redis_url.to_string()))
            .app_data(web::Data::new(redis_pool_connection.clone()))
            .app_data(web::Data::new(Tera::new("./templates/*").unwrap()))
            .service(Files::new("static", "./static"))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .configure(init_routes_pages)
            .configure(init_routes_redis)
    })
    .bind("0.0.0.0:8080")?
    .client_request_timeout(Duration::from_secs(config.get_request_timeout_sec()))
    .workers(config.get_workers())
    .run()
    .await
}
