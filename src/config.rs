pub mod database;

use std::env;
pub fn get_datbase_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}