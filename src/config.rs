#[derive(Clone)]
pub struct Config {
    #[allow(dead_code)]
    pub database_url: String,
    pub log_file: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            database_url: "sqlite://chop_shop.db".to_string(),
            log_file: "log.txt".to_string(),
        }
    }
}
