use actix_web::body::BoxBody;
use actix_web::{HttpResponse, ResponseError};
use actix_web::http::StatusCode;
use actix_web::middleware::ErrorHandlerResponse;
use diesel::internal::derives::multiconnection::chrono::{DateTime, Utc};
use serde::Serialize;
use thiserror::Error;
// use crate::utils::errors::AppError;

#[derive(Debug, Error)]
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

#[derive(Debug, Serialize)]
/// Represents a standard API response.
///
/// # Fields
/// - `success`: Indicates if the request was successful.
/// - `message`: A message providing additional information about the response.
/// - `data`: Optional data returned by the API.
/// - `context`: Contextual information about the response.
/// - `error`: Optional error information if the request was not successful.
pub struct ApiResponse<T>{
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub data: Option<T>,
    pub context: ResponseContext,
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<ApiError>,
}

#[derive(Debug, Serialize)]
pub struct ResponseContext {
    pub timestamp: DateTime<Utc>,
    pub user: Option<String>,
}


#[derive(Debug, Serialize)]
pub struct ApiError{
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub details: Option<serde_json::Value>
}



// Response builders
impl<T: Serialize> ApiResponse<T> {
    // 200 OK with data
    pub fn ok(data: T, message: impl Into<String>, user: Option<String>) -> HttpResponse {
        HttpResponse::Ok().json(Self {
            success: true,
            message: message.into(),
            data: Some(data),
            context: ResponseContext {
                timestamp: Utc::now(),
                user,
            },
            error: None,
        })
    }

    // 201 Created
    pub fn created(data: T, message: impl Into<String>, user: Option<String>) -> HttpResponse {
        HttpResponse::Created().json(Self {
            success: true,
            message: message.into(),
            data: Some(data),
            context: ResponseContext {
                timestamp: Utc::now(),
                user,
            },
            error: None,
        })
    }

    // 204 No Content
    pub fn no_content() -> HttpResponse {
        HttpResponse::NoContent().finish()
    }

    // 400 Bad Request
    pub fn bad_request(message: impl Into<String>, user: Option<String>) -> HttpResponse {
        let message  = message.into();
        HttpResponse::BadRequest().json(ApiResponse::<()> {
            success: false,
            message: message.clone(),
            data: None,
            context: ResponseContext {
                timestamp: Utc::now(),
                user,
            },
            error: Some(ApiError {
                code: "BAD_REQUEST".to_string(),
                message: message.clone(),
                details: None,
            }),
        })
    }

    // 401 Unauthorized
    pub fn unauthorized(message: impl Into<String>) -> HttpResponse {
        let message = message.into();
        HttpResponse::Unauthorized().json(ApiResponse::<()> {
            success: false,
            message: message.clone(),
            data: None,
            context: ResponseContext {
                timestamp: Utc::now(),
                user: None,
            },
            error: Some(ApiError {
                code: "UNAUTHORIZED".to_string(),
                message:  message.clone(),
                details: None,
            }),
        })
    }

    // 403 Forbidden
    pub fn forbidden(message: impl Into<String>, user: Option<String>) -> HttpResponse {
        let message = message.into();
        HttpResponse::Forbidden().json(ApiResponse::<()> {
            success: false,
            message: message.clone(),
            data: None,
            context: ResponseContext {
                timestamp: Utc::now(),
                user,
            },
            error: Some(ApiError {
                code: "FORBIDDEN".to_string(),
                message: message.clone(),
                details: None,
            }),
        })
    }

    // 404 Not Found
    pub fn not_found(message: impl Into<String>, user: Option<String>) -> HttpResponse {
        let message = message.into();

        HttpResponse::NotFound().json(ApiResponse::<()> {
            success: false,
            message: message.clone(),
            data: None,
            context: ResponseContext {
                timestamp: Utc::now(),
                user,
            },
            error: Some(ApiError {
                code: "NOT_FOUND".to_string(),
                message: message.clone(),
                details: None,
            }),
        })
    }

    // 422 Unprocessable Entity
    pub fn validation_error(message: impl Into<String>, details: serde_json::Value, user: Option<String>) -> HttpResponse {
        let message = message.into();

        HttpResponse::UnprocessableEntity().json(ApiResponse::<()> {
            success: false,
            message: message.clone(),
            data: None,
            context: ResponseContext {
                timestamp: Utc::now(),
                user,
            },
            error: Some(ApiError {
                code: "VALIDATION_ERROR".to_string(),
                message: message.clone(),
                details: Some(details),
            }),
        })
    }

    // 500 Internal Server Error
    pub fn internal_error(message: impl Into<String>, user: Option<String>) -> HttpResponse {
        let message = message.into();

        HttpResponse::InternalServerError().json(ApiResponse::<()> {
            success: false,
            message: message.clone(),
            data: None,
            context: ResponseContext {
                timestamp: Utc::now(),
                user,
            },
            error: Some(ApiError {
                code: "INTERNAL_ERROR".to_string(),
                message: message.clone(),
                details: None,
            }),
        })
    }

    // 503 Service Unavailable
    pub fn service_unavailable(message: impl Into<String>, user: Option<String>) -> HttpResponse {
        let message = message.into();

        HttpResponse::ServiceUnavailable().json(ApiResponse::<()> {
            success: false,
            message: message.clone(),
            data: None,
            context: ResponseContext {
                timestamp: Utc::now(),
                user,
            },
            error: Some(ApiError {
                code: "SERVICE_UNAVAILABLE".to_string(),
                message: message.clone(),
                details: None,
            }),
        })
    }

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