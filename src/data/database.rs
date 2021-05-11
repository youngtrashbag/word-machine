use lib::word::Word;

pub fn db_path() -> String {
    std::env::var("DB_PATH").expect("DB_PATH in environment file not set")
}
