use actix_web::{web, App, HttpServer};

pub mod www;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("").configure(www::config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}