use lib::word::Word;

// TODO: maybe use different sled::Tree for each language
// TODO: always opening and closing db connection in not good code, but it has to do for now
//          also, the application is not intended for big data traffic

fn db_path() -> String {
    std::env::var("DB_PATH").expect("DB_PATH in environment file not set")
}

/// get/fetch a word from database
pub fn select(word: &String) -> Result<Word, sled::Error> {
    let db = sled::open(db_path()).unwrap();

    match db.get(word.as_bytes())? {
        None => {
            warn!("{}", format!("Could not find Word \"{}\"", word));
            drop(db);

            Err(sled::Error::ReportableBug(format!("Could not find Word \"{}\"", word)))
        },
        Some(w) => {
            drop(db);
            Ok(Word::byte_deserialize(w.to_vec()))
        }
    }
}

/// insert or update word in db
pub fn insert(word: &Word) -> Result<Word, sled::Error> {
    let db = sled::open(db_path()).unwrap();

    match db.insert(word.word.as_bytes(), word.byte_serialize())? {
        None => {
            warn!("{}", format!("Could not insert/update Word \"{}\"", word.word));
            drop(db);

            Err(sled::Error::ReportableBug(format!("Could not insert/update Word \"{}\"", word.word)))
        },
        Some(w) => {
            drop(db);
            Ok(Word::byte_deserialize(w.to_vec()))
        }
    }
}

pub fn delete(word: &String) -> Result<Word, sled::Error> {
    let db = sled::open(db_path()).unwrap();

    match db.remove(word.as_bytes())? {
        None => {
            warn!("{}", format!("Could not delete Word \"{}\"", word));
            drop(db);

            Err(sled::Error::ReportableBug(format!("Could not delete Word \"{}\"", word)))
        },
        Some(w) => {
            drop(db);
            Ok(Word::byte_deserialize(w.to_vec()))
        }
    }
}

pub fn select_all() -> Result<Vec<Word>, sled::Error> {
    let db = sled::open(db_path()).unwrap();
    let mut words: Vec<Word> = Vec::new();

    for w in db.iter() {
        match w {
            Err(e) => {
                warn!("{}", format!("{}", e));
                drop(db);

                return Err(sled::Error::ReportableBug(e.to_string()));
            },
            Ok(w) => {
                words.push(Word::byte_deserialize(w.1.to_vec()));
            }
        }
    }

    return Ok(words);
}