use actix_web::{App, HttpServer, web, middleware};

#[macro_use]
extern crate log;

mod handlers;
mod parse;

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
    let port: String = std::env::var("PORT_FETCHING").expect("PORT_FETCHING in environment file not set");

    info!("Server Running on: {}:{}", ip_address, port);
    
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().header("Content-Type", "application/json"))

            .service(web::resource("/{word}").route(web::get().to(handlers::all_info)))
            //.service(web::resource("/{word}/definition").route(web::get().to(handlers::definition)))
            //.service(web::resource("/{word}/synonyms").route(web::get().to(handlers::definition)))

            .service(web::resource("/test").route(web::get().to(handlers::test)))
    })
    .bind(format!("{}:{}", ip_address, port))?
    .run()
    .await
}
