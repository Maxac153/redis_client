use actix_web::{post, web, HttpResponse, Responder};
use r2d2::{Pool, PooledConnection};
use r2d2_redis::{redis::Commands, RedisConnectionManager};
use serde::Deserialize;

use crate::models::response::Response;

#[derive(Deserialize)]
pub struct AddListRequest {
    key: String,
    add_mode: String,
}

async fn add_to_redis(
    con: &mut PooledConnection<RedisConnectionManager>,
    key: &str,
    data: &str,
    add_mode: &str,
) -> Result<(), String> {
    match add_mode {
        "FIRST" => con.lpush(key, data).map_err(|e| e.to_string()),
        "LAST" => con.rpush(key, data).map_err(|e| e.to_string()),
        _ => Err(format!(
            "Incorrect operation ({}), expected (FIRST, LAST)!",
            add_mode
        )),
    }
}

#[post("/addList")]
pub async fn add_list(
    pool: web::Data<Pool<RedisConnectionManager>>,
    query: web::Query<AddListRequest>,
    data: String,
) -> impl Responder {
    let mut con: PooledConnection<RedisConnectionManager> = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            return HttpResponse::InternalServerError().json(Response::error(
                "Failed to get a connection from the pool!".to_string(),
            ))
        }
    };

    match add_to_redis(&mut con, &query.key, &data, &query.add_mode).await {
        Ok(_) => HttpResponse::Created().json(Response::ok(
            "Data added successfully.".to_string(),
            "".to_string(),
        )),
        Err(err) => HttpResponse::BadRequest().json(Response::error(err)),
    }
}

#[derive(Deserialize)]
pub struct AddSetRequest {
    key: String,
    field: String,
}

#[post("/addHash")]
pub async fn add_hash(
    pool: web::Data<Pool<RedisConnectionManager>>,
    query: web::Query<AddSetRequest>,
    data: String,
) -> impl Responder {
    let mut con: PooledConnection<RedisConnectionManager> = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            return HttpResponse::InternalServerError().json(Response::error(
                "Failed to get a connection from the pool!".to_string(),
            ));
        }
    };

    let res: HttpResponse = match con.hset::<&str, &str, String, ()>(&query.key, &query.field, data){
        Ok(_) => HttpResponse::Created().json(Response::ok(
            "Data added successfully.".to_string(),
            "".to_string(),
        )),
        Err(err) => HttpResponse::BadRequest().json(Response::error(err.to_string())),
    };

    res
}