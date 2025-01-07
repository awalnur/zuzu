use actix_web::dev::HttpServiceFactory;
use actix_web::web;
use crate::handlers::user_handler::{ list_users, create_user};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/user")
                .service(list_users)
                .service(create_user)
            )
    );
}
