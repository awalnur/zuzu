use actix_web::{HttpResponse, ResponseError};
use actix_web::dev::ServiceResponse;
use actix_web::http::StatusCode;
use actix_web::middleware::{ErrorHandlerResponse};
use thiserror::Error;
use crate::utils::response::ApiResponse;

#[derive(Debug, Error, Clone)]
pub enum AppError{
    #[error("Record not found: {0}")]
    NotFound(String),
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("Validation Error: {0}")]
    ValidationError(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Internal Error: {0}")]
    InternalError(String),
    #[error("Service Unavailable: {0}")]
    ServiceUnavailable(String),
}
impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::ValidationError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
        }
    }
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::NotFound(message) => ApiResponse::<()>::not_found(message.clone(), None),
            AppError::BadRequest(message) => ApiResponse::<()>::bad_request(message.clone(), None),
            AppError::ValidationError(message) => ApiResponse::<()>::validation_error(message.clone(), serde_json::Value::Null, None),
            AppError::Unauthorized(message) => ApiResponse::<()>::unauthorized(message.clone()),
            AppError::InternalError(message) => ApiResponse::<()>::internal_error(message.clone(), None),
            AppError::Forbidden(message) => ApiResponse::<()>::forbidden(message.clone(), None),
            AppError::ServiceUnavailable(message) => ApiResponse::<()>::service_unavailable(message.clone(), None),
        }
    }
}


pub(crate) fn error_handler<B>(service_response: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>, actix_web::Error>
where
    B: 'static
{
    // Extract the request from the response
    let error = service_response.response().error()
        .and_then(|e| e.as_error::<AppError>())
        .cloned()
        .unwrap_or_else(||
            match service_response.response().status() {
                StatusCode::INTERNAL_SERVER_ERROR => AppError::ServiceUnavailable("Service Temporary Unavailable".to_string()),
                StatusCode::BAD_REQUEST => AppError::BadRequest("Invalid input".to_string()),
                StatusCode::NOT_FOUND => AppError::NotFound("Resource not found".to_string()),
                StatusCode::UNPROCESSABLE_ENTITY => AppError::ValidationError("Validation Error".to_string()),
                StatusCode::UNAUTHORIZED => AppError::Unauthorized("Unauthorized Access".to_string()),
                StatusCode::FORBIDDEN => AppError::Forbidden("Access Denied".to_string()),
                StatusCode::SERVICE_UNAVAILABLE => AppError::ServiceUnavailable("Service Temporary Unavailable".to_string()),
                _ => AppError::ServiceUnavailable("Service Temporary Unavailable".to_string()),
            }
        );

    // let error_response = error.error_response();
    // Match on the error type for detailed handling
    let error_response = match error {
        AppError::BadRequest(details) => {
            AppError::BadRequest(format!("Invalid input: {}", details)).error_response()
        }
        AppError::Unauthorized(details) => {
            AppError::Unauthorized(format!("Access denied: {}", details)).error_response()
        }
        AppError::NotFound(details) => {
            AppError::NotFound(format!("Resource missing: {}", details)).error_response()
        }
        AppError::InternalError(details) => {
            AppError::ServiceUnavailable(format!("Server issue: {}", details)).error_response()
        }
        _ => {
            AppError::ServiceUnavailable("Unexpected error occurred".to_string()).error_response()
        }
    };

    let (request, _) = service_response.into_parts();
    // Convert to the correct response type
    let service_response = ServiceResponse::new(
        request,
        error_response.map_into_boxed_body()
    );

    // Convert to EitherBody
    let service_response = service_response.map_into_right_body();

    Ok(ErrorHandlerResponse::Response(service_response))
}
