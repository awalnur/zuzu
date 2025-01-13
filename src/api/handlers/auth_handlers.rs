use actix_web::{get, post, web, HttpResponse};
use crate::api::dto::requests::auth::RegisterRequest;
use crate::api::dto::responses::{ApiResponse, AuthResponse};
use crate::config::database::DbPool;
use crate::domain::models::authentication::LoginRequest;
use crate::domain::services::authentication;

#[post("/create_user")]
pub async fn create_user(
    pool: web::Data<DbPool>,
    user: web::Json<RegisterRequest>,
) -> actix_web::Result<HttpResponse> {
    let user = user.into_inner();
    Ok(ApiResponse::ok(
        user,
        "User created successfully",
        None,
    ))
}


#[post("/token")]
pub async fn generate_token(
    pool: web::Data<DbPool>,
    user: web::Json<LoginRequest>,
) -> actix_web::Result<HttpResponse> {
    let user = authentication::generate_token(pool, user).await.map_err(|e| {
        actix_web::error::ErrorUnauthorized(e.to_string())
    })?;
    let auth = AuthResponse{
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

#[get("/all")]
pub async fn list_auth(pool: web::Data<DbPool>) -> actix_web::Result<HttpResponse> {
    // let query = accounts.select(User::as_select());
    // let mut conn = pool
    //     .get()
    //     .map_err(|e| actix_web::error::ErrorServiceUnavailable(e))?;
    // let users = web::block(move || {
    //     let data = query.load::<User>(&mut conn).expect("Error loading users");
    //     data
    // })
    //     .await?;

    Ok(ApiResponse::ok(
        "res_data",
        "Users fetched successfully",
        None,
    ))
}

