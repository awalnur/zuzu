
use actix_web::{web, App, HttpServer, middleware::Logger, middleware};
use actix_web::http::StatusCode;
use actix_web::middleware::{ErrorHandlers};
use diesel::{Connection, RunQueryDsl, SelectableHelper};

pub mod handlers;
pub mod models;
pub mod schemas;
pub mod config;
pub mod utils;
pub mod middlewares;
// pub mod routes;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let pool = match config::database::init_pool(){
        Ok(pool) => Some(pool),
        Err(e) => {
            eprintln!("Failed to create pool: {}", e);
            None
        },
    };
    HttpServer::new(move||
                        {
                            let mut app = App::new()
                                // .wrap(middlewares::error_middleware::configure_error_handlers())
                                .wrap(Logger::default())
                                .configure(routes::user_routes::init);
                            if pool.is_some(){
                                app = app.data(pool.clone().unwrap());
                            }
                            app
                        })
        .bind("127.0.0.1:8082")?
        .run()
        .await
    }


