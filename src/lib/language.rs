use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
pub enum Language {
    English,
    Deutsch,
}

impl Language {
    pub fn from_string(lang: &str) -> Option<Language> {
        match lang {
            "de" => Some(Language::Deutsch),
            "en" => Some(Language::English),
            _ => None
        }
    }
}

impl ToString for Language {
    fn to_string(&self) -> String {
        let mut lang: String = String::new();

        match self {
            Language::Deutsch => lang = "de".to_string(),
            Language::English => lang = "en".to_string()
        };

        lang
    }
}
