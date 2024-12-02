use actix_web::{get, web, HttpResponse};
use tera::Tera;

#[get("/")]
pub async fn index(tera: web::Data<Tera>, redis_url: web::Data<String>) -> HttpResponse {
    let mut context = tera::Context::new();
    context.insert("redis_url", redis_url.as_ref());
    HttpResponse::Ok().body(tera.render("index.html", &context).unwrap())
}
