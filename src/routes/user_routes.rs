

use actix_web::web;
use api_with_rust::handlers::user_handler::get_user;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/users", web::get().to(get_user))
            // .route("/users/create", web::post().to(create_user))
    );
}
