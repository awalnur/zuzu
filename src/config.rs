pub mod database;
pub mod error_handling;

use std::env;
pub fn get_datbase_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}