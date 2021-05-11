use lib::word::Word;

// TODO: maybe use different sled::Tree for each language
// TODO: always opening and closing db connection in not good code, but it has to do for now
//          also, the application is not intended for big data traffic

pub fn db_path() -> String {
    std::env::var("DB_PATH").expect("DB_PATH in environment file not set")
}
