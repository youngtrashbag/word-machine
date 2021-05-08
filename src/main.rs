fn main() {
    match std::env::var("RUST_LOG") {
        Err(_) => {
            std::env::set_var("RUST_LOG", "info");
        },
        Ok(_) => {()}
    };

    env_logger::init();

    dotenv::dotenv().ok();

    let db_path = std::env::var("DB_PATH").expect("DB_PATH in environment file not set");
    let db = sled::open(db_path).unwrap();
}
