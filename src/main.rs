use actix_web::{App, HttpServer, web, HttpResponse, middleware};
use actix_cors::Cors;

#[macro_use]
extern crate log;

mod word;
mod language;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match std::env::var("RUST_LOG") {
        Err(_) => {
            std::env::set_var("RUST_LOG", "info");
        },
        Ok(_) => {()}
    };

    env_logger::init();

    dotenv::dotenv().ok();

    let db_path = std::env::var("DB_PATH").expect("DB_PATH in environment file not set");
    let _ = sled::open(db_path).unwrap();

    let ip_address: String = "127.0.0.1".to_string();
    let port: String = "4322".to_string();

    info!("Server Running on: {}:{}", ip_address, port);
    
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .wrap(middleware::DefaultHeaders::new().header("Content-Type", "application/json"))

    })
    .bind(format!("{}:{}", ip_address, port))?
    .run()
    .await
}
