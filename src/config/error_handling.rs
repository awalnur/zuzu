use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::middleware::ErrorHandlers;
use crate::utils::errors::error_handler;

pub fn init_error_handlers() -> ErrorHandlers<BoxBody> {
    ErrorHandlers::<BoxBody>::new()
        .handler(StatusCode::INTERNAL_SERVER_ERROR, error_handler)
        .handler(StatusCode::BAD_REQUEST, error_handler)
        .handler(StatusCode::NOT_FOUND, error_handler)
        .handler(StatusCode::UNPROCESSABLE_ENTITY, error_handler)
        .handler(StatusCode::UNAUTHORIZED, error_handler)
        .handler(StatusCode::FORBIDDEN, error_handler)
        .handler(StatusCode::SERVICE_UNAVAILABLE, error_handler)
}