use actix_web::web;

use crate::handlers::pages::index::index;

pub fn init_routes_pages(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}
