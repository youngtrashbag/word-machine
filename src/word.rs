use serde::{Serialize, Deserialize};

use crate::language::Language;

#[derive(Serialize,Deserialize)]
struct Word {
    /// the word itself
    pub word: String,
    /// list of synonyms (maybe synonyms will have None as this field)
    pub synonyms: Option<Vec<Word>>,
    /// meaning of the word
    pub definition: String,
    /// the language of the word
    pub language: Language,
}

impl Word {
    /// serialize into byte vector, in order to save object in database
    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }

    /// deserialize from byte vector, so object can be read from database
    pub fn deserialize(json: Vec<u8>) -> Self {
        bincode::deserialize(&json).unwrap()
    }
}
