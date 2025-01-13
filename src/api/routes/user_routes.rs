use actix_web::web;
use crate::api::handlers::user_handlers::{list_users};
use crate::api::handlers::auth_handlers::{create_user, generate_token, list_auth};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/user")
                .service(list_users)
            )
            .service(web::scope("/auth")
                .service(create_user)
                .service(list_auth)
                .service(generate_token)
            )
    );
}
