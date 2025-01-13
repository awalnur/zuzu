use chrono::{DateTime, Utc};
use serde::Serialize;

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
pub struct AuthResponse{
    pub access_token: String,
    pub refresh_token: String,
    pub expires: i64,
    pub token_type: Option<String>,
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



