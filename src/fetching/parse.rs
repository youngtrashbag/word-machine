use serde::{Serialize, Deserialize};
use serde_json::{Value};

use lib::word::Word;
use lib::language::Language;

pub fn parse(json: String) -> Word {
    let v: Value = serde_json::from_str(&json).unwrap();

    info!("word {}", v[0]["meta"]["id"].as_str().unwrap());

    Word {
        word: v[0]["meta"]["id"].as_str().unwrap().to_string(),
        synonyms: Some(serde_json::from_value::<Vec<String>>(v[0]["meta"]["syns"][0].clone()).unwrap()),
        definition: v[0]["shortdef"][0].as_str().unwrap().to_string(),
        language: Language::English,
    }
}