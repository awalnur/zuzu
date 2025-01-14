use crate::{config::database::DbPool};
use actix_web::{delete, get, post, put, web, HttpResponse};
use crate::api::dto::responses::ApiResponse;
use crate::domain::models::user::NewUser;
use crate::domain::services::user_services;

#[get("/all")]
pub async fn list_users(pool: web::Data<DbPool>) -> actix_web::Result<HttpResponse> {
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

#[post("/create")]
pub async fn create_user(
    pool: web::Data<DbPool>,
    user: web::Json<NewUser>,
) -> actix_web::Result<HttpResponse> {
    let userdata = user.into_inner();
    let create_user = user_services::create_user(pool, userdata).await?;
    Ok(ApiResponse::ok(
        create_user,
        "User created successfully",
        None,
    ))
}

#[get("/users/{id}")]
pub async fn get_user_by_id(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
) -> actix_web::Result<HttpResponse> {
    let id = id.into_inner();
    let user = user_services::find_user_by_id(pool, id).await?;
    Ok(ApiResponse::ok(
        user,
        "User fetched successfully",
        None,
    ))
}

#[put("/update/{id}")]
pub async fn update_user(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
    user: web::Json<NewUser>,
) -> actix_web::Result<HttpResponse> {
    let id = id.into_inner();
    let user = user.into_inner();
    let updated_user = user_services::update_user(pool, id, user).await?;
    Ok(ApiResponse::ok(
        updated_user,
        "User updated successfully",
        None,
    ))
}

#[delete("/delete/{id}")]
pub async fn delete_user(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
) -> actix_web::Result<HttpResponse> {
    let id = id.into_inner();
    user_services::delete_user(pool, id).await?;
    Ok(ApiResponse::ok(
        "res_data",
        "User deleted successfully",
        None,
    ))
}

// #[get("/users/{id}")]
// pub async fn get_user_by_id(
//     pool: web::Data<DbPool>,
//     uname: web::Path<String>,
// ) -> actix_web::Result<impl Responder> {
//     let uname = uname.into_inner();
//
//     let user = web::block(move || {
//         let mut conn = pool.get()?;
//         accounts
//             .filter(username.eq(uname))
//             // .find(user_id)
//             .select(User::as_select())
//             .first::<User>(&mut conn)
//         })
//         .await?.map_err(actix_web::error::ErrorInternalServerError)?;
//
//     Ok(HttpResponse::Ok().json(user))
// }
//

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::config::database::init_pool;
//     use actix_web::{test, App};
//
//     #[actix_rt::test]
//     async fn list_users_returns_empty_list_when_no_users() {
//         let pool = init_pool();
//         let mut app = test::init_service(
//             App::new()
//                 .app_data(web::Data::new(pool.clone()))
//                 .service(list_users),
//         )
//         .await;
//
//         let req = test::TestRequest::get().uri("/all").to_request();
//         let resp = test::call_service(&mut app, req).await;
//         let body: serde_json::Value = test::read_body_json(resp).await;
//
//         assert!(body["data"]["entries"].is_array());
//         assert_eq!(body["data"]["entries"].as_array().unwrap().len(), 0);
//     }
//
//     // #[actix_rt::test]
//     // async fn create_user_creates_user_successfully() {
//     //     let pool = init_pool();
//     //     let mut app = test::init_service(App::new().app_data(web::Data::new(pool.clone())).service(create_user)).await;
//     //
//     //     let new_user = NewUser {
//     //         username: "testuser".to_string(),
//     //         email: "testuser@example.com".to_string(),
//     //         password: "password".to_string(),
//     //     };
//     //
//     //     let req = test::TestRequest::post().uri("/create").set_json(&new_user).to_request();
//     //     let resp = test::call_service(&mut app, req).await;
//     //     let body: serde_json::Value = test::read_body_json(resp).await;
//     //
//     //     assert_eq!(body["data"]["username"], "testuser");
//     //     assert_eq!(body["data"]["email"], "testuser@example.com");
//     // }
//
//     #[actix_rt::test]
//     async fn create_user_fails_with_invalid_data() {
//         let pool = init_pool();
//         let mut app = test::init_service(
//             App::new()
//                 .app_data(web::Data::new(pool.clone()))
//                 .service(create_user),
//         )
//         .await;
//
//         let invalid_user = serde_json::json!({
//             "username": "",
//             "email": "invalidemail",
//             "password": "short"
//         });
//
//         let req = test::TestRequest::post()
//             .uri("/create")
//             .set_json(&invalid_user)
//             .to_request();
//         let resp = test::call_service(&mut app, req).await;
//
//         assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);
//     }
// }
