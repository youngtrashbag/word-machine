use actix_web::{HttpResponse, web};
use reqwest::blocking::{get, Client};

use lib::reqwest::build_client;
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

    info!("Fetching word \"{}\" from Thesaurus", word);
    let res = client
        .get(format!("https://dictionaryapi.com/api/v3/references/thesaurus/json/{}?key={}", word, api_key()))
        .send().unwrap();

    let word = parse(res.text().unwrap());

    HttpResponse::Ok()
        .body(serde_json::to_string(&word).unwrap())
}
