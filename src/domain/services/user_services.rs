use actix_web::{web, Error};
use actix_web::web::Data;
use uuid::Uuid;
use crate::config::database::DbPool;
use crate::domain::models::user::{NewUser, User};
use crate::domain::repository::Repository;
use crate::domain::repositories::user_repository::UserRepository;

pub(crate) async fn create_user(
    pool: web::Data<DbPool>,
    user: NewUser) -> Result<User, Error> {
    let user = UserRepository::new(pool).create(user).await?;
    Ok(user)
}

pub(crate) async fn list_users(
    pool: Data<DbPool>,
    limit: Option<i16>,
    offsets: Option<i16>,
    search: Option<String>,
) -> Result<Vec<User>, Error> {
    let users = UserRepository::new(pool).find_all(limit, offsets, search).await?;
    Ok(users)
}
pub(crate) async fn find_user_by_id(
    pool: Data<DbPool>,
    id: String,
) -> Result<User, Error> {
    let id =Uuid::parse_str(&*id.to_string()).unwrap();
    let user = UserRepository::new(pool).find_by_id(id).await?;
    Ok(user)
}

pub(crate) async fn update_user(
    pool: Data<DbPool>,
    id: String,
    user: NewUser,
) -> Result<User, Error> {
    let id =Uuid::parse_str(&*id.to_string()).unwrap();

    let user = UserRepository::new(pool).update(id, user).await?;
    Ok(user)
}

pub(crate) async fn delete_user(
    pool: Data<DbPool>,
    id: String,
) -> Result<(), Error> {
    let id =Uuid::parse_str(&*id.to_string()).unwrap();

    UserRepository::new(pool).delete(id).await?;
    Ok(())
}