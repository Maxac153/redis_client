use crate::handlers::pages::index::index;

use actix_web::web;

pub fn init_routes_pages(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}
