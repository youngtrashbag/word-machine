use actix_web::{HttpResponse, web};
use lib::reqwest::build_client;

pub async fn test(web::Path(word): web::Path<String>) -> HttpResponse {
    HttpResponse::Ok()
        .body("hello world")
}

pub async fn all_info(web::Path(word): web::Path<String>) -> HttpResponse {
    let req_client = build_client();

    HttpResponse::Ok()
        .body("hello world")
}
