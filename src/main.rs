use actix_web::{App, HttpServer};

pub mod mvc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(mvc::configure_controllers)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}