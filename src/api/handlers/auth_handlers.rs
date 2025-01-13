use actix_web::{get, post, web, HttpResponse};
use crate::api::dto::requests::auth::RegisterRequest;
use crate::api::dto::responses::ApiResponse;
use crate::config::database::DbPool;
use crate::domain::models::authentication::LoginRequest;
use crate::domain::services::authentication;
use crate::infrastructure::database::schemas::schemas::password_hashes::user_id;

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

    Ok(ApiResponse::ok(
        user,
        "Login successful",
        None,
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

