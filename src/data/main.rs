use actix_web::{App, HttpServer, web, middleware};
use actix_cors::Cors;

#[macro_use]
extern crate log;

mod database;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match std::env::var("RUST_LOG") {
        Err(_) => {
            std::env::set_var("RUST_LOG", "info");
        },
        Ok(_) => ()
    };

    env_logger::init();

    dotenv::dotenv().ok();

    let ip_address: String = "127.0.0.1".to_string();
    let port: String = std::env::var("REACT_APP_PORT_DATA").expect("REACT_APP_PORT_DATA in environment file not set");

    info!("Server Running on: {}:{}", ip_address, port);
    
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .wrap(middleware::DefaultHeaders::new().header("Content-Type", "application/json"))

            .service(web::resource("/word/{language}/{word}")
                .route(web::get().to(handlers::get_word))
                .route(web::delete().to(handlers::delete_word)))
            .service(web::resource("/word/{language}").route(web::post().to(handlers::new_word)))
            .service(web::resource("/all_words/{language}").route(web::get().to(handlers::all_words)))

            .service(web::resource("/test").route(web::get().to(handlers::test)))
    })
    .bind(format!("{}:{}", ip_address, port))?
    .run()
    .await
}
