use crate::models::response::Response;

use actix_web::{post, Responder};
use actix_web::{web, HttpResponse};
use r2d2::{Pool, PooledConnection};
use r2d2_redis::{
    redis::{self, Commands, RedisResult},
    RedisConnectionManager,
};
use serde::Deserialize;
use std::collections::HashMap;

fn restore(con: &mut redis::Connection, key: String, data: Vec<u8>) -> RedisResult<String> {
    let status: String = redis::cmd("RESTORE").arg(key).arg(0).arg(data).query(con)?;
    Ok(status)
}

#[derive(Deserialize)]
pub struct UploadDumpRequest {
    key_name: String,
}

#[utoipa::path(
    tag = "Redis Client",
    description = "Redis Upload Dump Key - Загрузка дампа в Redis",
    post,
    path = "/uploadDumpKey",
    params(
        ("key_name" = String, Query, description = "Name of the key", example = "listKey")
    ),
    request_body = Vec<u8>,
)]
#[post("/uploadDumpKey")]
pub async fn upload_dump_key(
    pool: web::Data<Pool<RedisConnectionManager>>,
    query: web::Query<UploadDumpRequest>,
    bytes: web::Bytes,
) -> impl Responder {
    let mut con: PooledConnection<RedisConnectionManager> = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            return HttpResponse::InternalServerError().json(Response::error(
                "Failed to get a connection from the pool!".to_string(),
            ));
        }
    };

    if bytes.is_empty() {
        return HttpResponse::BadRequest()
            .json(Response::error("Uploaded file is empty.".to_string()));
    }

    match con.del::<&String, ()>(&query.key_name) {
        Ok(_) => {}
        Err(err) => return HttpResponse::Ok().json(Response::error(err.to_string())),
    };

    let res = match restore(&mut con, query.key_name.clone(), bytes.to_vec()) {
        Ok(_) => Response::ok("Files uploaded successfully!".to_string(), "".to_string()),
        Err(err) => Response::error(err.to_string()),
    };

    HttpResponse::Ok().json(res)
}

fn vec_to_hashmap(data: Vec<u8>) -> Result<HashMap<String, Vec<u8>>, String> {
    let mut result: HashMap<String, Vec<u8>> = HashMap::new();
    let mut index: usize = 0;

    while index < data.len() {
        if index + 4 > data.len() {
            return Err("Insufficient data for key length".to_string());
        }

        let key_length: usize =
            u32::from_le_bytes(data[index..index + 4].try_into().unwrap()) as usize;
        index += 4;

        if index + key_length > data.len() {
            return Err("Insufficient data for key".to_string());
        }

        let key_result = String::from_utf8(data[index..index + key_length].to_vec());
        let key = match key_result {
            Ok(k) => k,
            Err(_) => return Err("Invalid UTF-8 sequence in key".to_string()),
        };
        index += key_length;

        if index + 4 > data.len() {
            return Err("Insufficient data for value length".to_string());
        }

        let value_length: usize =
            u32::from_le_bytes(data[index..index + 4].try_into().unwrap()) as usize;
        index += 4;

        if index + value_length > data.len() {
            return Err("Insufficient data for value".to_string());
        }

        let value: Vec<u8> = data[index..index + value_length].to_vec();
        index += value_length;

        result.insert(key, value);
    }

    Ok(result)
}

#[utoipa::path(
    tag = "Redis Client",
    description = "Redis Upload Dump All Keys - Загрузка дампа со всеми ключами в Redis",
    post,
    path = "/uploadDumpAllKeys",
    request_body = Vec<u8>,
)]
#[post("/uploadDumpAllKeys")]
pub async fn upload_dump_all_keys(
    pool: web::Data<Pool<RedisConnectionManager>>,
    bytes: web::Bytes,
) -> impl Responder {
    let mut con: PooledConnection<RedisConnectionManager> = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            return HttpResponse::BadRequest().json(Response::error(
                "Failed to get a connection from the pool!".to_string(),
            ));
        }
    };

    if bytes.is_empty() {
        return HttpResponse::BadRequest()
            .json(Response::error("Uploaded file is empty.".to_string()));
    }

    let dump: HashMap<String, Vec<u8>> = match vec_to_hashmap(bytes.to_vec()) {
        Ok(contents) => contents,
        Err(_) => {
            return HttpResponse::InternalServerError().json(Response::error(
                "An error was signalled by the server: DUMP payload version or checksum are wrong"
                    .to_string(),
            ));
        }
    };

    for (key, data) in dump {
        if let Err(err) = con.del::<String, ()>(key.clone()) {
            return HttpResponse::Ok().json(Response::error(err.to_string()));
        }

        if let Err(err) = restore(&mut con, key.clone(), data) {
            return HttpResponse::Ok().json(Response::error(err.to_string()));
        }
    }

    HttpResponse::Ok().json(Response::ok(
        "Files uploaded successfully!".to_string(),
        "".to_string(),
    ))
}
