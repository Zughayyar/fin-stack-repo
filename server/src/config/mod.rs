use dotenv::dotenv;
use std::env;

pub mod errors;

pub fn get_database_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn get_server_url() -> String {
    dotenv().ok();
    env::var("SERVER_URL").unwrap_or_else(|_| "127.0.0.1:8080".to_string())
} 