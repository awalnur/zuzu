use actix_web::{HttpResponse, Responder};
use diesel::{r2d2, PgConnection};
use diesel::r2d2::ConnectionManager;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn get_user( ) -> actix_web::Result<impl Responder> {

    Ok(HttpResponse::Ok().json({}))
}