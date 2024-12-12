use crate::models::response::Response;

use actix_web::{delete, web, HttpResponse, Responder};
use r2d2::{Pool, PooledConnection};
use r2d2_redis::{redis::Commands, RedisConnectionManager};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ResetKeyRequest {
    key: String,
}

#[utoipa::path(
    tag = "Redis Client",
    description = "Redis Reset Key - Удалить данные из базы с определённым ключом",
    delete,
    path = "/resetKey",
    params(
        ("key" = String, Query, description = "Name of the key", example = "listKey"),
    ),
)]
#[delete("/resetKey")]
pub async fn reset_key(
    pool: web::Data<Pool<RedisConnectionManager>>,
    query: web::Query<ResetKeyRequest>,
) -> impl Responder {
    let mut con: PooledConnection<RedisConnectionManager> = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            return HttpResponse::InternalServerError().json(Response::error(
                "Failed to get a connection from the pool!".to_string(),
            ));
        }
    };

    let exists: bool = match con.exists::<&String, bool>(&query.key) {
        Ok(exists) => exists,
        Err(err) => return HttpResponse::BadRequest().json(Response::error(err.to_string())),
    };

    if !exists {
        return HttpResponse::BadRequest().json(Response::error(format!(
            "The key ({}) does not exist!",
            query.key
        )));
    }

    let res: HttpResponse = match con.del::<&String, ()>(&query.key) {
        Ok(_) => HttpResponse::Ok().json(Response::ok(
            format!("Deleted record key ({}).", query.key),
            "".to_string(),
        )),
        Err(err) => HttpResponse::BadRequest().json(Response::error(err.to_string())),
    };

    res
}

#[utoipa::path(
    tag = "Redis Client",
    description = "Redis Reset All Keys - Удалить все данные из Redis",
    delete,
    path = "/resetAllKeys"
)]
#[delete("/resetAllKeys")]
pub async fn reset_all_keys(pool: web::Data<Pool<RedisConnectionManager>>) -> impl Responder {
    let mut con: PooledConnection<RedisConnectionManager> = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            return HttpResponse::InternalServerError().json(Response::error(
                "Failed to get a connection from the pool!".to_string(),
            ));
        }
    };

    let keys: r2d2_redis::redis::Iter<'_, _> = match con.scan() {
        Ok(keys) => keys,
        Err(err) => return HttpResponse::BadRequest().json(Response::error(err.to_string())),
    };

    let keys_to_delete: Vec<String> = keys.collect();
    for key in keys_to_delete {
        match con.del::<&String, ()>(&key) {
            Ok(_) => {}
            Err(err) => return HttpResponse::BadRequest().json(Response::error(err.to_string())),
        };
    }

    HttpResponse::Ok().json(Response::ok(
        "Deleted all keys successfully.".to_string(),
        "".to_string(),
    ))
}
