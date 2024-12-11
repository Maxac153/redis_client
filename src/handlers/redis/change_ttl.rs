use actix_web::{patch, web, HttpResponse, Responder};
use r2d2::{Pool, PooledConnection};
use r2d2_redis::{redis::Commands, RedisConnectionManager};

use crate::models::response::Response;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ChangeTtlRequest {
    key: String,
    ttl: usize,
}

#[utoipa::path(
    tag = "Redis Client",
    description = "Redis Change TTL - Смена TTL (изменение времени жизни данных)",
    patch,
    path = "/changeTtl",
    params(
        ("key" = String, Query, description = "Name of the key", example = "listKey"),
        ("ttl" = String, Query, description = "TTL - Time to live", example = "600")
    ),
)]
#[patch("/changeTtl")]
pub async fn change_ttl(
    pool: web::Data<Pool<RedisConnectionManager>>,
    query: web::Query<ChangeTtlRequest>,
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
            "Key ({}) does not exist!",
            &query.key
        )));
    }

    let res: Response = if query.ttl == 0 {
        match con.persist::<&String, ()>(&query.key) {
            Ok(_) => Response::ok(
                format!("Expiration removed for key ({}).", &query.key),
                "".to_string(),
            ),
            Err(err) => return HttpResponse::BadRequest().json(Response::error(err.to_string())),
        }
    } else {
        match con.expire::<&String, usize>(&query.key, query.ttl) {
            Ok(_) => Response::ok(
                format!(
                    "TTL successfully changed, key ({}), ttl ({} sec).",
                    query.key, query.ttl
                ),
                "".to_string(),
            ),
            Err(err) => return HttpResponse::BadRequest().json(Response::error(err.to_string())),
        }
    };

    HttpResponse::Ok().json(res)
}
