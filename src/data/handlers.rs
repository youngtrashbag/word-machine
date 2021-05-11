use actix_web::{HttpResponse, web};

use lib::word::Word;
use lib::language::Language;
use lib::Message;
use crate::database;

pub async fn get_word(web::Path(word): web::Path<String>) -> HttpResponse {
    match database::select(&word) {
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

pub async fn new_word(new_word: web::Json<Word>) -> HttpResponse {
    let new_word = new_word.into_inner();

    #[allow(unused_must_use)]
    database::insert(&new_word);
    match database::select(&new_word.word) {
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
    match database::delete(&word) {
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

pub async fn all_words() -> HttpResponse {
    match database::select_all() {
        Err(e) => {
            HttpResponse::InternalServerError()
                .body(Message::new(&e.to_string()).to_json())
        },
        Ok(v) => {
            HttpResponse::Ok()
                .body(serde_json::to_string(&v).unwrap())
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

    match database::insert(&word) {
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
