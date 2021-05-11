use serde::{Serialize, Deserialize};
use serde_json::{Value};

use lib::word::Word;
use lib::language::Language;

pub fn parse(json: String) -> Word {
    let v: Value = serde_json::from_str(&json).unwrap();

    Word {
        word: v[0]["meta"]["id"].to_string(),
        synonyms: Some(serde_json::from_value::<Vec<String>>(v[0]["meta"]["syns"][0].clone()).unwrap()),
        definition: v[0]["def"][0]["sseq"][0][0][1]["dt"][0][1].to_string(),
        language: Language::English,
    }
}