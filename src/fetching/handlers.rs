use actix_web::{HttpResponse, web};

pub async fn test(web::Path(word): web::Path<String>) -> HttpResponse {
    HttpResponse::Ok()
        .body("hello world")
}
