use actix_web::{get, web, Responder};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}

#[get("/")]
pub async fn index() -> impl Responder {
    "Index"
}