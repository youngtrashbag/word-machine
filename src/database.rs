use crate::word::Word;

// TODO: maybe use different sled::Tree for each language

impl Word {
    /// fetch a word from database
    pub fn get(word: String) -> Result<Word, sled::Error> {
        let db = sled::Db;

        match db.get(word.as_bytes())? {
            None => {
                warn!("{}", format!("Could not find Word \"{}\"", word));

                Err(sled::Error::ReportableBug(format!format!("Could not find Word \"{}\"", word)))
            },
            Some(w) => {
                Ok(Word::deserialize(w.to_vec()))
            }
        }
    }

    /// insert or update word in db
    pub fn update(word: Word) -> Result<Word, sled::Error> {
        let db = sled::Db;

        match db.insert(word.word.as_bytes())? {
            None => {
                warn!("{}", format!("Could not insert/update Word \"{}\"", word));

                Err(sled::Error::ReportableBug(format!format!("Could not insert/update Word \"{}\"", word)))
            },
            Some(w) => {
                Ok(Word::deserialize(w.to_vec()))
            }
        }
    }

    pub fn delete(word: String) -> Result<Word, sled::Error> {
        let db = sled::Db;

        match db.remove(word.as_bytes())? {
            None => {
                warn!("{}", format!("Could not delete Word \"{}\"", word));

                Err(sled::Error::ReportableBug(format!("Could not delete Word \"{}\"", word)))
            },
            Some(w) => {
                Ok(Word::deserialize(w.to_vec()))
            }
        }
    }
}