use actix_web::{HttpResponse, web};

use lib::reqwest::build_client;
use lib::Message;
use crate::parse::parse;

fn api_key() -> String {
    std::env::var("THESAURUS_API_KEY").expect("THESAURUS_API_KEY in environment file not set")
}

pub async fn test() -> HttpResponse {
    HttpResponse::Ok()
        .body("hello world")
}

pub async fn all_info(web::Path(word): web::Path<String>) -> HttpResponse {
    let client = build_client();

    info!("Fetching word {} from Thesaurus", word);
    let res = client
        .get(format!("https://dictionaryapi.com/api/v3/references/thesaurus/json/{}?key={}", word, api_key()))
        .send().unwrap();

    match parse(res.text().unwrap()) {
        None => {
            HttpResponse::NotFound()
                .body(Message::new(&format!("Could not find word: {}", word)).to_json())
        },
        Some(w) => {
            HttpResponse::Ok()
                .body(serde_json::to_string(&w).unwrap())
        }
    }

}
