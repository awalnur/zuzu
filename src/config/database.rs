use diesel::r2d2::ConnectionManager;
use diesel::{   PgConnection};
use dotenvy::dotenv;
use std::env;
use crate::utils::response::AppError;

// Database connection configuration
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
// Initialize the database pool
pub fn init_pool() -> Result<DbPool, AppError> {
    // Load the environment variables
    dotenv().ok();

    // Get the database URL from the environment
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);

    // Create the database pool with the connection managers.
    r2d2::Pool::builder()
    .build(manager)
        .map_err(|e| AppError::ServiceUnavailable(format!("Failed to create pool: {}", e)))


}
