use actix_web::{
    error,
    HttpResponse
    };

use diesel::internal::derives::multiconnection::chrono::{DateTime, Utc};
use serde::Serialize;
use thiserror::Error;


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

