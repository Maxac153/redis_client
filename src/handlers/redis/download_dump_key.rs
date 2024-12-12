use crate::models::response::Response;

use actix_web::{get, web, HttpResponse, Responder};
use r2d2::{Pool, PooledConnection};
use r2d2_redis::{
    redis::{self, Commands, RedisResult},
    RedisConnectionManager,
};
use serde::Deserialize;
use std::collections::HashMap;

fn dump_key(con: &mut redis::Connection, key: &str) -> RedisResult<Vec<u8>> {
    let dump_key: Vec<u8> = redis::cmd("DUMP").arg(key).query(con)?;
    Ok(dump_key)
}

#[derive(Deserialize)]
pub struct DownloadDumpRequest {
    key: String,
}

#[utoipa::path(
    tag = "Redis Client",
    description = "Redis Download Dump Key - Скачивание дампа по ключу из Redis",
    get,
    path = "/downloadDumpKey",
    params(
        ("key" = String, Query, description = "Name of the key", example = "listKey"),
    ),
)]
#[get("/downloadDumpKey")]
pub async fn download_dump_key(
    pool: web::Data<Pool<RedisConnectionManager>>,
    query: web::Query<DownloadDumpRequest>,
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
        return HttpResponse::NotFound().json(Response::error(format!(
            "Key ({}) does not exist!",
            query.key
        )));
    }

    match dump_key(&mut con, &query.key) {
        Ok(dump) => HttpResponse::Ok()
            .content_type("application/octet-stream")
            .insert_header((
                "Content-Disposition",
                format!("attachment; filename=\"{}.dump\"", query.key),
            ))
            .body(dump),
        Err(err) => return HttpResponse::BadRequest().json(Response::error(err.to_string())),
    }
}

fn hashmap_to_vec(hashmap: HashMap<String, Vec<u8>>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    for (key, value) in hashmap {
        let key_bytes: &[u8] = key.as_bytes();
        let key_length: u32 = key_bytes.len() as u32;

        result.extend_from_slice(&key_length.to_le_bytes());
        result.extend_from_slice(key_bytes);

        let value_length: u32 = value.len() as u32;
        result.extend_from_slice(&value_length.to_le_bytes());
        result.extend_from_slice(&value);
    }

    result
}

#[utoipa::path(
    tag = "Redis Client",
    description = "Redis Download Dump All Keys - Скачивание дампа всех ключей из Redis",
    get,
    path = "/downloadDumpAllKeys"
)]
#[get("/downloadDumpAllKeys")]
pub async fn download_dump_all_keys(
    pool: web::Data<Pool<RedisConnectionManager>>,
) -> impl Responder {
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
        Err(err) => return HttpResponse::Ok().json(Response::error(err.to_string())),
    };

    let keys: Vec<String> = keys.collect();

    if keys.is_empty() {
        return HttpResponse::NotFound()
            .json(Response::error("No keys found in Redis!".to_string()));
    }

    let mut key_file: HashMap<String, Vec<u8>> = HashMap::new();

    for key in keys {
        match dump_key(&mut con, &key) {
            Ok(dump) => {
                key_file.insert(key, dump);
            }
            Err(err) => {
                return HttpResponse::BadRequest().json(Response::error(format!(
                    "Error dumping key {}: {}",
                    key, err
                )));
            }
        }
    }

    HttpResponse::Ok()
        .content_type("application/octet-stream")
        .insert_header((
            "Content-Disposition",
            "attachment; filename=\"dump_all_keys.rdb\"",
        ))
        .body(hashmap_to_vec(key_file))
}
