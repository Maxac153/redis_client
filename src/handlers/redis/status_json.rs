use actix_web::{get, web, HttpResponse, Responder};
use r2d2::{Pool, PooledConnection};
use r2d2_redis::{
    redis::{self, Commands, FromRedisValue, Iter, RedisResult, ToRedisArgs},
    RedisConnectionManager,
};
use serde::Deserialize;

use crate::models::{
    response::Response, status::StatusJson, status_key::StatusKey, type_key::TypeKey,
};

fn total_memory_usage(con: &mut redis::Connection) -> redis::RedisResult<String> {
    let memory_usage_info: String = redis::cmd("INFO").arg("MEMORY").query(con)?;
    memory_usage_info
        .lines()
        .find(|line: &&str| line.starts_with("used_memory_human:"))
        .and_then(|line: &str| line.split(':').nth(1))
        .map(|value: &str| value.trim().to_string())
        .ok_or_else(|| {
            redis::RedisError::from((redis::ErrorKind::ResponseError, "No memory usage found!"))
        })
}

fn connected_clients(con: &mut redis::Connection) -> redis::RedisResult<u32> {
    let connected_info: String = redis::cmd("INFO").arg("CLIENTS").query(con)?;
    connected_info
        .lines()
        .nth(1)
        .and_then(|line: &str| line.split(':').nth(1))
        .and_then(|value: &str| value.trim().parse::<u32>().ok())
        .ok_or_else(|| {
            redis::RedisError::from((redis::ErrorKind::ResponseError, "No client info found!"))
        })
}

fn handle_redis_error<T>(result: Result<T, redis::RedisError>) -> Result<T, HttpResponse> {
    result.map_err(|err| HttpResponse::InternalServerError().json(Response::error(err.to_string())))
}

fn get_keys<'a, String: ToRedisArgs, RV: FromRedisValue>(
    con: &'a mut redis::Connection,
    search_key: String,
    type_key: &str,
) -> RedisResult<Iter<'a, RV>> {
    let mut c: redis::Cmd = redis::cmd("SCAN");
    c.arg(0)
        .arg("MATCH")
        .arg(search_key)
        .arg("COUNT")
        .arg(10000)
        .arg("TYPE")
        .arg(type_key);
    c.iter(con)
}

fn key_memory_usage(con: &mut redis::Connection, key: &str) -> redis::RedisResult<u32> {
    redis::cmd("MEMORY").arg("USAGE").arg(key).query(con)
}

#[derive(Debug, Deserialize)]
pub struct StatusJsonRequest {
    search_key: Option<String>,
    type_key: Option<TypeKey>,
    lower_limit: usize,
    upper_limit: usize,
}

#[get("/statusJson")]
pub async fn status_json(
    pool: web::Data<Pool<RedisConnectionManager>>,
    query: web::Query<StatusJsonRequest>,
) -> impl Responder {
    if query.lower_limit >= query.upper_limit {
        return HttpResponse::BadRequest().json(Response::error(
            "Lower_limit must be less than upper_limit!".to_string(),
        ));
    }

    if query.upper_limit - query.lower_limit > 30 {
        return HttpResponse::BadRequest().json(Response::error(
            "The difference between upper_limit and lower_limit must not exceed 30!".to_string(),
        ));
    }

    let mut con: PooledConnection<RedisConnectionManager> = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            return HttpResponse::InternalServerError().json(Response::error(
                "Failed to get a connection from the pool!".to_string(),
            ));
        }
    };

    let num_clients: u32 = match handle_redis_error(connected_clients(&mut con)) {
        Ok(clients) => clients,
        Err(err) => {
            return HttpResponse::BadRequest().json(Response::error(format!(
                "Error retrieving number of connected clients: {:?}",
                err
            )));
        }
    };

    let total_memory_usage: String = match handle_redis_error(total_memory_usage(&mut con)) {
        Ok(memory_usage) => memory_usage,
        Err(err) => {
            return HttpResponse::BadRequest().json(Response::error(format!(
                "Error retrieving total memory usage: {:?}",
                err
            )));
        }
    };

    let search_key: String = format!(
        "*{}*",
        query.search_key.clone().unwrap_or_else(|| "".to_string())
    );

    let type_key = match query.type_key {
        Some(ref key) => key.as_str(),
        None => "list",
    };

    let keys_iter = match handle_redis_error(get_keys(&mut con, &search_key, &type_key)) {
        Ok(keys) => keys,
        Err(err) => {
            return HttpResponse::BadRequest().json(Response::error(format!(
                "Error retrieving keys for search key ({}) and type key ({}) : {:?}",
                search_key, type_key, err
            )));
        }
    };
    let keys: Vec<String> = keys_iter.collect();

    let filtered_keys: Vec<String> = keys
        .iter()
        .skip(query.lower_limit)
        .take(query.upper_limit - query.lower_limit)
        .cloned()
        .collect();

    let statuses: Vec<StatusKey> = filtered_keys
        .iter()
        .filter_map(|key: &String| {
            let len_result = match type_key {
                "list" => handle_redis_error(con.llen(key)),
                "hash" => handle_redis_error(con.hlen(key)),
                _ => Ok(0),
            };

            let memory_usage_result = handle_redis_error(key_memory_usage(&mut con, key));
            let ttl_result = handle_redis_error(con.pttl(key));

            if let (Ok(len), Ok(memory_usage), Ok(ttl)) =
                (len_result, memory_usage_result, ttl_result)
            {
                Some(
                    StatusKey::default()
                        .key(key)
                        .len(len)
                        .memory_usage(memory_usage)
                        .ttl(ttl),
                )
            } else {
                None
            }
        })
        .collect();

    HttpResponse::Ok().json(
        StatusJson::default()
            .connected_clients(num_clients)
            .total_memory_usage(total_memory_usage)
            .keys(keys)
            .statuses(statuses),
    )
}

fn get_type_key(con: &mut redis::Connection, key: &str) -> redis::RedisResult<String> {
    redis::cmd("TYPE").arg(key).query(con)
}

#[derive(Debug, Deserialize)]
pub struct StatusKeyRequest {
    search_key: String,
}

#[get("/statusKey")]
pub async fn status_key(
    pool: web::Data<Pool<RedisConnectionManager>>,
    query: web::Query<StatusKeyRequest>,
) -> impl Responder {
    let key = query.search_key.clone();
    let mut con: PooledConnection<RedisConnectionManager> = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            return HttpResponse::InternalServerError().json(Response::error(
                "Failed to get a connection from the pool!".to_string(),
            ));
        }
    };

    let exists: bool = match con.exists(&key) {
        Ok(exists) => exists,
        Err(_) => {
            return HttpResponse::BadRequest().json(Response::error(
                "Failed to check if the key exists.".to_string(),
            ));
        }
    };

    if !exists {
        return HttpResponse::BadRequest()
            .json(Response::error(format!("Key ({}) does not exist!", key)));
    }

    let type_key: String = match get_type_key(&mut con, &key) {
        Ok(type_key) => type_key,
        Err(err) => {
            return HttpResponse::BadRequest().json(Response::error(format!(
                "Error retrieving type key: {}",
                err
            )));
        }
    };

    let len_result = match type_key.as_str() {
        "list" => handle_redis_error(con.llen(&key)),
        "hash" => handle_redis_error(con.hlen(&key)),
        _ => Ok(0),
    };

    let len: i32 = match len_result {
        Ok(length) => length,
        Err(err) => {
            return HttpResponse::BadRequest().json(Response::error(format!(
                "Error retrieving length for key ({}) of type ({}) : {:?}",
                key, type_key, err
            )));
        }
    };

    let key_memory_usage: u32 = match handle_redis_error(key_memory_usage(&mut con, &key)) {
        Ok(memory_usage) => memory_usage,
        Err(err) => {
            return HttpResponse::BadRequest().json(Response::error(format!(
                "Error retrieving memory usage for key ({}) : {:?}",
                key, err
            )));
        }
    };

    let ttl: i32 = match handle_redis_error(con.pttl(&key)) {
        Ok(ttl_value) => ttl_value,
        Err(err) => {
            return HttpResponse::BadRequest().json(Response::error(format!(
                "Error retrieving TTL for key ({}) : {:?}",
                key, err
            )));
        }
    };

    HttpResponse::Ok().json(
        StatusKey::default()
            .key(&key)
            .type_key(type_key.as_str().into())
            .len(len)
            .memory_usage(key_memory_usage)
            .ttl(ttl),
    )
}
