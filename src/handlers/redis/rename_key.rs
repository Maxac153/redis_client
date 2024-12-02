use actix_web::{patch, web, HttpResponse, Responder};
use r2d2::{Pool, PooledConnection};
use r2d2_redis::{redis::Commands, RedisConnectionManager};

use crate::models::response::Response;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RenameKeyRequest {
    old_name_key: String,
    new_name_key: String,
}

#[patch("/renameKey")]
pub async fn rename_key(
    pool: web::Data<Pool<RedisConnectionManager>>,
    query: web::Query<RenameKeyRequest>,
) -> impl Responder {
    let mut con: PooledConnection<RedisConnectionManager> = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            return HttpResponse::InternalServerError().json(Response::error(
                "Failed to get a connection from the pool!".to_string(),
            ));
        }
    };

    let exists: bool = match con.exists::<&String, bool>(&query.new_name_key) {
        Ok(exists) => exists,
        Err(err) => return HttpResponse::BadRequest().json(Response::error(err.to_string())),
    };

    if exists {
        return HttpResponse::BadRequest().json(Response::error(format!(
            "A key ({}) with this name already exists!",
            query.new_name_key
        )));
    }

    let res: HttpResponse = match con
        .rename::<String, String>(query.old_name_key.clone(), query.new_name_key.clone())
    {
        Ok(_) => HttpResponse::Ok().json(Response::ok(
            format!(
                "Key ({} -> {}) successfully renamed",
                query.old_name_key, query.new_name_key
            ),
            "".to_string(),
        )),
        Err(err) => HttpResponse::BadRequest().json(Response::error(err.to_string())),
    };

    res
}
