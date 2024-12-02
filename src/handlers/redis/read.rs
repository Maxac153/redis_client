use std::collections::HashMap;

use actix_web::{get, web, HttpResponse, Responder};
use r2d2::{Pool, PooledConnection};
use r2d2_redis::{
    redis::{Commands, RedisError},
    RedisConnectionManager,
};
use serde::Deserialize;
use serde_json::{json, to_string_pretty};

use crate::models::response::Response;

#[derive(Deserialize)]
pub struct ReadListRequest {
    key: String,
    read_mode: String,
}

async fn read_from_redis(
    con: &mut PooledConnection<RedisConnectionManager>,
    key: &str,
    read_mode: &str,
) -> Result<Option<String>, String> {
    match read_mode {
        "FIRST" => con.lpop(key).map_err(|e| e.to_string()),
        "LAST" => con.rpop(key).map_err(|e| e.to_string()),
        _ => Err(format!(
            "Incorrect operation ({}), expected (FIRST, LAST)!",
            read_mode
        )),
    }
}

#[get("/readList")]
pub async fn read_list(
    pool: web::Data<Pool<RedisConnectionManager>>,
    query: web::Query<ReadListRequest>,
) -> impl Responder {
    let mut con: PooledConnection<RedisConnectionManager> = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            return HttpResponse::InternalServerError().json(Response::error(
                "Failed to get a connection from the pool!".to_string(),
            ))
        }
    };

    match read_from_redis(&mut con, &query.key, &query.read_mode).await {
        Ok(Some(data)) => {
            HttpResponse::Ok().json(Response::ok("Data read successfully.".to_string(), data))
        }
        Ok(None) => HttpResponse::BadRequest().json(Response::error(format!(
            "The key '{}' does not exist or is empty!",
            query.key
        ))),
        Err(err) => HttpResponse::BadRequest().json(Response::error(err)),
    }
}

fn hashmap_to_json_string(map: HashMap<String, String>) -> String {
    let mut json_object = json!({});

    for (key, value) in map.iter() {
        json_object[key] = json!({
            "data": value
        });
    }

    to_string_pretty(&json_object).unwrap()
}

#[derive(Deserialize)]
pub struct ReadHashRequest {
    key: String,
}

#[get("/readHash")]
pub async fn read_hash(
    pool: web::Data<Pool<RedisConnectionManager>>,
    query: web::Query<ReadHashRequest>,
) -> impl Responder {
    let mut con: PooledConnection<RedisConnectionManager> = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            return HttpResponse::InternalServerError().json(Response::error(
                "Failed to get a connection from the pool!".to_string(),
            ));
        }
    };

    let result: Result<HashMap<String, String>, RedisError> = con.hgetall(&query.key);
    match result {
        Ok(data) if !data.is_empty() => HttpResponse::Ok().json(Response::ok(
            "Data read successfully.".to_string(),
            hashmap_to_json_string(data),
        )),
        Ok(_) => HttpResponse::NotFound().json(Response::error("Hash not found!".to_string())),
        Err(err) => HttpResponse::BadRequest().json(Response::error(err.to_string())),
    }
}
