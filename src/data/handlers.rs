#[allow(unused_must_use)]
use actix_web::{HttpResponse, web};

use lib::word::Word;
use lib::language::Language;
use lib::Message;
use lib::reqwest::build_client;

use crate::database::db_path;

/// word does not yet exist; fetch from other service, save to db, return word (201 created)
/// word exists, return word
pub async fn get_word(web::Path((language, word)): web::Path<(String, String)>) -> HttpResponse {
    let db = sled::open(db_path()).unwrap();

    // parsing language for security
    let lang = Language::from_string(&language).unwrap();
    let db = db.open_tree(lang.to_string().as_bytes()).unwrap();

    match db.get(word.as_bytes()).unwrap() {
        None => {
            //warn!("{}", format!("Could not find Word {}", word));

            let client = build_client();

            let res = client.get(
                format!("http://localhost:{}/{}",
                    std::env::var("PORT_FETCHING").expect("PORT_FETCHING in environment file not set"),
                    word
                )
            )
            .send().unwrap();

            if res.status().is_client_error() {
                return HttpResponse::NotFound()
                    .body(res.text().unwrap());
            }

            let w: Word = serde_json::from_str(&res.text().unwrap()).unwrap();

            // inserting
            db.insert(word.as_bytes(), w.byte_serialize());
            // reding word from db
            match db.get(word.as_bytes()) {
                Err(e) => {
                    drop(db);
                    return HttpResponse::NotFound()
                        .body(Message::new(&e.to_string()).to_json());
                },
                Ok(w) => {
                    drop(db);
                    let w = Word::byte_deserialize(w.unwrap().to_vec());

                    return HttpResponse::Created()
                        .body(serde_json::to_string(&w).unwrap());
                }
            }
        },
        Some(w) => {
            drop(db);
            let w = Word::byte_deserialize(w.to_vec());

            return HttpResponse::Ok()
                .body(serde_json::to_string(&w).unwrap());
        }
    }
}

pub async fn new_word(web::Path(language): web::Path<String>, new_word: web::Json<Word>) -> HttpResponse {
    let db = sled::open(db_path()).unwrap();
    let new_word = new_word.into_inner();

    let lang = Language::from_string(&language).unwrap();
    let db = db.open_tree(lang.to_string().as_bytes()).unwrap();

    db.insert(new_word.word.as_bytes(), new_word.byte_serialize());
    match db.get(new_word.word.as_bytes()).unwrap() {
        None => {
            let msg = format!("Could not find Word {}", new_word.word);
            warn!("{}", msg);
            drop(db);

            HttpResponse::NotFound()
                .body(Message::new(&msg).to_json())
        },
        Some(w) => {
            drop(db);
            let word = Word::byte_deserialize(w.to_vec());

            HttpResponse::Created()
                .body(serde_json::to_string(&word).unwrap())
        }
    }
}

pub async fn delete_word(web::Path((language, word)): web::Path<(String, String)>) -> HttpResponse {
    let db = sled::open(db_path()).unwrap();

    let lang = Language::from_string(&language).unwrap();
    let db = db.open_tree(lang.to_string().as_bytes()).unwrap();

    match db.remove(word.as_bytes()).unwrap() {
        None => {
            let msg = format!("Could not delete Word {}", word);
            warn!("{}", msg);
            drop(db);

            HttpResponse::InternalServerError()
                .body(Message::new(&msg).to_json())
        },
        Some(w) => {
            drop(db);
            let w = Word::byte_deserialize(w.to_vec());

            HttpResponse::Gone()
                .body(serde_json::to_string(&w).unwrap())
        }
    }
}

pub async fn all_words(web::Path(language): web::Path<String>) -> HttpResponse {
    let db = sled::open(db_path()).unwrap();

    let lang = Language::from_string(&language).unwrap();
    let db = db.open_tree(lang.to_string().as_bytes()).unwrap();

    let mut words: Vec<Word> = Vec::new();

    for w in db.iter() {
        match w {
            Err(e) => {
                warn!("{}", format!("{}", e));
                drop(db);

                return HttpResponse::InternalServerError()
                    .body(Message::new(&e.to_string()).to_json())
            },
            Ok(w) => {
                words.push(Word::byte_deserialize(w.1.to_vec()));
            }
        }
    }

    HttpResponse::Ok()
        .body(serde_json::to_string(&words).unwrap())
}

pub async fn test() -> HttpResponse {
    let word = Word {
        word: "soliloquy".to_string(),
        synonyms: Some(vec!["discourse".to_string(), "monologue".to_string()]),
        definition: "the act of talking to oneself".to_string(),
        language: Language::English,
    };

    HttpResponse::Ok()
        .body(serde_json::to_string(&word).unwrap())
}
