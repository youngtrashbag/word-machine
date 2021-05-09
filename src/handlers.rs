use actix_web::{HttpResponse, web};

use crate::word::Word;
use crate::language::Language;
use crate::utils::Message;

pub async fn get_word(web::Path(word): web::Path<String>) -> HttpResponse {
    match Word::get(word) {
        Err(e) => {
            HttpResponse::NotFound()
                .body(Message::new(&e.to_string()).to_json())
        },
        Ok(w) => {
            HttpResponse::Ok()
                .body(serde_json::to_string(&w).unwrap())
        }
    }
}

pub async fn new_word(new_word: web::Form<Word>) -> HttpResponse {
    match Word::update(new_word.into_inner()) {
        Err(e) => {
            HttpResponse::InternalServerError()
                .body(Message::new(&e.to_string()).to_json())
        },
        Ok(w) => {
            HttpResponse::Ok()
                .body(serde_json::to_string(&w).unwrap())
        }
    }
}

pub async fn delete_word(web::Path(word): web::Path<String>) -> HttpResponse {
    match Word::delete(word) {
        Err(e) => {
            HttpResponse::InternalServerError()
                .body(Message::new(&e.to_string()).to_json())
        },
        Ok(w) => {
            HttpResponse::Ok()
                .body(serde_json::to_string(&w).unwrap())
        }
    }
}

pub async fn test() -> HttpResponse {
        let word = Word {
            word: "soliloquy".to_string(),
            synonyms: Some(vec!["discourse".to_string(), "monologue".to_string()]),
            definition: "the act of talking to oneself".to_string(),
            language: Language::English,
        };

        match Word::update(word) {
        Err(e) => {
            HttpResponse::NotFound()
                .body(Message::new(&e.to_string()).to_json())
        },
        Ok(w) => {
            HttpResponse::Ok()
                .body(serde_json::to_string(&w).unwrap())
        }
    }
}
