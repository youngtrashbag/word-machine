use reqwest::blocking::Response;

use lib::reqwest::build_client;
use lib::word::Word;
use lib::language::Language;

pub fn fetch(lang: &Language, word: &String) -> Response {
    let client = build_client();

    match lang {
        Language::English => {
            let res = client
                .get(format!("https://dictionaryapi.com/api/v3/references/thesaurus/json/{}?key={}",
                    word,
                    std::env::var("THESAURUS_API_KEY").expect("THESAURUS_API_KEY in environment file not set")
                ))
                .send().unwrap();

            res
        },
        Language::Deutsch => {
            let res = client
                .get(format!("https://api.dictionaryapi.dev/api/v2/entries/de/{}", word))
                .send().unwrap();

            res
        }
    }
}
