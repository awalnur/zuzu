use actix_web::web;
use crate::services::user::{list_users, create_user};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/user")
                .service(list_users)
                .service(create_user)
            )
    );
}
