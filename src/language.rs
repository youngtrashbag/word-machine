use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
pub enum Language {
    English,
    Deutsch,
}
