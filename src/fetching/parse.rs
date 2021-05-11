use serde::{Serialize, Deserialize};
use serde_json::{Value};

use lib::word::Word;
use lib::language::Language;

pub fn parse(json: String) -> Word {
    let v: Value = serde_json::from_str(&json).unwrap();

    Word {
        word: v[0]["meta"]["id"].to_string(),
        synonyms: Some(serde_json::from_value::<Vec<String>>(v[0]["meta"]["syns"][0].clone()).unwrap()),
        definition: v[0]["shortdef"][0].to_string(),
        language: Language::English,
    }
}