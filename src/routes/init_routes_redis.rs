use actix_web::web;

use crate::handlers::redis::add::{add_hash, add_list};
use crate::handlers::redis::change_ttl::change_ttl;
use crate::handlers::redis::download_dump_key::{download_dump_all_keys, download_dump_key};
use crate::handlers::redis::read::{read_hash, read_list};
use crate::handlers::redis::rename_key::rename_key;
use crate::handlers::redis::reset::{reset_all_keys, reset_key};
use crate::handlers::redis::status::{status_json, status_key};
use crate::handlers::redis::upload_dump_key::{upload_dump_all_keys, upload_dump_key};

pub fn init_routes_redis(cfg: &mut web::ServiceConfig) {
    cfg.service(status_json)
        .service(status_key)
        .service(add_list)
        .service(add_hash)
        .service(read_list)
        .service(read_hash)
        .service(change_ttl)
        .service(rename_key)
        .service(reset_key)
        .service(reset_all_keys)
        .service(download_dump_key)
        .service(download_dump_all_keys)
        .service(upload_dump_key)
        .service(upload_dump_all_keys);
}
