use actix_web::{HttpResponse, web};

use lib::Message;
use lib::language::Language;
use crate::fetching::{*};
use crate::parse::{*};


pub async fn all_info(web::Path((language, word)): web::Path<(String, String)>) -> HttpResponse {
    let lang = match Language::from_string(&language) {
        None => {
            return HttpResponse::BadRequest()
                .body(Message::new(&format!("Language: {} not supported", language)).to_json());
        },
        Some(l) => l,
    };

    let res = fetch(&lang, &word);
    match parse(&lang, res.text().unwrap()) {
        None => {
            HttpResponse::NotFound()
                .body(Message::new(&format!("Could not find word: {}", word)).to_json())
        },
        Some(w) =>{
            HttpResponse::Ok()
                .body(serde_json::to_string(&w).unwrap())
        }
    }
}
