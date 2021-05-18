use serde_json::{Value};

use lib::word::Word;
use lib::language::Language;

pub fn parse(lang: &Language, json: String) -> Option<Word> {
    let v: Value = serde_json::from_str(&json).unwrap();

    match lang {
        Language::English => {
            match v[0]["meta"]["id"].as_str() {
                None => None,
                Some(w) => {
                    Some(Word {
                        word: w.to_string(),
                        synonyms: Some(serde_json::from_value::<Vec<String>>(v[0]["meta"]["syns"][0].clone()).unwrap()),
                        definition: v[0]["shortdef"][0].as_str().unwrap().to_string(),
                        language: Language::English,
                    })
                },
            }
        },
        Language::Deutsch => {
            match v[0]["word"].as_str() {
                None => None,
                Some(w) => {
                    let mut synonyms: Vec<String> = Vec::new();
                    let mut antonyms: Vec<String> = Vec::new();
                    let mut definitions: Vec<String> = Vec::new();

                    // nice closure func
                    let append_to = | v1: Option<&Vec<Value>>, v2: &mut Vec<String> | {
                        if v1.is_some() {
                            for e in v1.unwrap() {
                                v2.push(e.as_str().unwrap().to_string());
                            }
                        }
                    };

                    // array
                    for element in v[0]["meanings"][0]["definitions"].clone().as_array().unwrap() {
                        append_to(element["synonyms"].as_array(), &mut synonyms);
                        append_to(element["antonyms"].as_array(), &mut antonyms);

                        if let Some(d) = element["definition"].as_str() {
                            definitions.push(d.to_string());
                        }
                    }

                    Some(Word {
                        word: w.to_string(),
                        synonyms: Some(synonyms),
                        definition: definitions.first().unwrap().to_string(),
                        language: Language::Deutsch,
                    })
                },
            }

        }

    }
}
