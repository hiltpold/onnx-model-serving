mod error;
mod handler;
mod model;
mod utils;
mod yolo;
use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::ACCEPT])
            .supports_credentials();
        App::new()
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await?;
    Ok(())
}
