use actix_web::{ web, App, HttpServer, middleware::Logger};
use diesel::{r2d2, PgConnection};
use r2d2::ConnectionManager;

// use api_with_rust::routes;

mod routes;
mod config;
// use api_with_rust

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let pool = config::database::init_pool();
    // connect to SQLite DB
    let manager = ConnectionManager::<PgConnection>::new("localhost:5432/lumbung");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file");

    HttpServer::new(move|| App::new()
        .app_data(web::Data::new(pool.clone()))
        .wrap(Logger::default())
        .configure(routes::user_routes::init))
        .bind("127.0.0.1:8082")?
        .run()
        .await
}


