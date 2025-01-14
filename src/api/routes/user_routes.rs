use actix_web::web;
use crate::api::handlers::user_handlers::{create_user, delete_user, list_users, update_user};
use crate::api::handlers::auth_handlers::{generate_token,list_auth};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/user")
                .service(list_users)
                .service(create_user)
                .service(update_user)
                .service(delete_user)
            )
            .service(web::scope("/auth")
                .service(list_auth)
                .service(generate_token)
            )
    );
}
