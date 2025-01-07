use std::collections::HashMap;
use actix_web::{get, HttpResponse, Responder, web, post, Error};
use actix_web::middleware::ErrorHandlers;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper, prelude::*};
use diesel::pg::Pg;
use diesel::query_builder::AsQuery;
use crate::{
    models::user::User,
    config::database::DbPool,
    schemas::schemas::accounts::dsl::*,
};
use crate::models::user::NewUser;
use crate::utils::response::ApiResponse;


#[get("/all")]
pub async fn list_users(
    pool: web::Data<DbPool>,
) -> actix_web::Result<HttpResponse> {
    let query = accounts.select(User::as_select());
    let mut conn = pool.get().expect("Connection Error");

    let users = web::block(move || {
        let data = query.load::<User>(&mut conn).expect("Error loading users");
        data
    })
        .await?;

    let mut res_data = HashMap::new();
    res_data.insert("entries", &users);

    Ok(ApiResponse::ok(res_data, "Users fetched successfully", None))

}

#[post("/create")]
pub async fn create_user(
    pool: web::Data<DbPool>,
    user: web::Json<NewUser>,
) -> actix_web::Result<HttpResponse> {
    let user = user.into_inner();
    let created_user = web::block(move || {
        let mut conn = pool.get().expect("Connection Error");
        diesel::insert_into(accounts)
            .values(&user)
            .get_result::<User>(&mut conn).expect("Error creating user")
    })
        .await?;
    Ok(ApiResponse::ok(created_user, "User created successfully", None))
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


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    use crate::config::database::init_pool;

    #[actix_rt::test]
    async fn list_users_returns_users_successfully() {
        let pool = init_pool();
        let mut app = test::init_service(App::new().app_data(web::Data::new(pool.clone())).service(list_users)).await;

        let req = test::TestRequest::get().uri("/all").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
        let body: serde_json::Value = test::read_body_json(resp).await;
        assert!(body["data"]["entries"].is_array());
    }

    #[actix_rt::test]
    async fn list_users_handles_empty_database() {
        let pool = init_pool();
        let mut app = test::init_service(App::new().app_data(web::Data::new(pool.clone())).service(list_users)).await;

        let req = test::TestRequest::get().uri("/all").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
        let body: serde_json::Value = test::read_body_json(resp).await;
        // assert_ne!(body["entries"].as_array().unwrap().len(), 0);
        assert!(body["data"]["entries"].as_array().unwrap().len()>= 1);
    }
}

