use crate::api::dto::responses::AuthResponse;
use crate::config::database::DbPool;
use crate::domain::models::authentication::LoginRequest;
use crate::domain::services::authentication;
use actix_web::{post, web, HttpResponse};

#[post("/token")]
pub async fn generate_token(
    pool: web::Data<DbPool>,
    user: web::Json<LoginRequest>,
) -> actix_web::Result<HttpResponse> {
    let user = authentication::token(pool, user)
        .await
        .map_err(|e| actix_web::error::ErrorUnauthorized(e.to_string()))?;
    let auth = AuthResponse {
        access_token: user.get("access_token").unwrap().to_string(),
        refresh_token: user.get("refresh_token").unwrap().to_string(),
        expires: user.get("expires").unwrap().parse().unwrap(),
        token_type: Option::from(user.get("token_type").unwrap().to_string()),
    };
    Ok(AuthResponse::ok(
        auth.access_token,
        auth.refresh_token,
        auth.expires,
        auth.token_type.unwrap(),
    ))
}

// #[post("/register")]
// pub async fn register_email(
//     pool :web::Data<DbPool>,
//     data: web::Json<String>
// )-> actix_web::Result<HttpResponse>{
//     let res = authentication::register(pool, data)
// }
