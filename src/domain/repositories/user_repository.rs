use actix_web::{web, Error};
use diesel::{debug_query, ExpressionMethods, JoinOnDsl, NullableExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper, Table};
use diesel::associations::HasTable;

use uuid::Uuid;
use crate::config::database::DbPool;
use crate::domain::models::user::{NewUser, PasswordHash, User};
use crate::domain::repository::Repository;
use crate::infrastructure::database::schemas::schemas::accounts::dsl;
use crate::infrastructure::database::schemas::schemas::accounts::dsl::accounts;
use crate::infrastructure::database::schemas::schemas::password_hashes;
use crate::utils::crypto::Password;
use crate::utils::errors::AppError;

pub struct UserRepository{
    pool: web::Data<DbPool>
}

impl UserRepository{
    pub fn new(pool: web::Data<DbPool>) -> Self{
        UserRepository{
            pool
        }
    }
}
#[async_trait::async_trait]
impl Repository<User, Uuid, NewUser, NewUser,(User, Option<PasswordHash>)> for UserRepository{
    async fn find_all(&self, limit: Option<i16>, offset: Option<i16>, search: Option<String>) -> Result<Vec<User>, Error> {
        let pool = self.pool.clone();
        let query  = accounts.select(User::as_select());
        let mut conn = pool
            .get()
            .map_err(|e| AppError::ServiceUnavailable(e.to_string()))?;

        let users = web::block(move || {
            let data = query.load::<User>(&mut conn).expect("Error loading users");
            data
        }).await?;

        Ok(users)

    }

    async fn find_by_id(&self, id: Uuid) -> Result<User, Error> {
        let query = accounts.filter(dsl::id.eq(id));
        let mut conn = self.pool.get().map_err(|e| AppError::ServiceUnavailable(e.to_string()))?;
        let user = web::block(move || {
            query.first::<User>(&mut conn).map_err(|e| AppError::NotFound(e.to_string()))
        }).await?;
        Ok(user?)
    }

    async fn find_by(&self, id: Uuid) -> Result<User, Error> {
        todo!()
    }

    async fn find_by_email(&self, email: &str) -> Result<User, Error> {
        todo!()
    }
    async fn find_by_username(&self, username: &str) -> Result<(User, Option<PasswordHash>), Error> {
        let username = username.to_string();
        let query = accounts.left_outer_join(password_hashes::dsl::password_hashes::table().on(dsl::id.eq(password_hashes::dsl::user_id))).filter(dsl::username.eq(username)).select((User::as_select(),
            (password_hashes::dsl::id.nullable(),
            password_hashes::dsl::user_id.nullable(),
            password_hashes::dsl::password_hash.nullable(),
            password_hashes::dsl::salt.nullable(),
            password_hashes::dsl::algorithm.nullable(),
            password_hashes::dsl::is_temporary.nullable(),
            password_hashes::dsl::expiry.nullable(),
            password_hashes::dsl::last_change_at.nullable(),
            password_hashes::dsl::created_at.nullable(),
            password_hashes::dsl::updated_at.nullable()
            ).nullable(),
        ));
        let mut conn = self.pool.get().map_err(|e| AppError::ServiceUnavailable(e.to_string()))?;
        let user = web::block(move || {
            query.first::<(User, Option<PasswordHash>)>(&mut conn).map_err(|e| AppError::NotFound(e.to_string()))

        }).await?;
        Ok(user?)
    }



    async fn create(&self, data: NewUser) -> Result<User, Error> {
        todo!()
    }

    async fn update(&self, id: Uuid, entry: NewUser) -> Result<User, Error> {
        todo!()
    }

    async fn delete(&self, id: Uuid) -> Result<(), Error> {
        todo!()
    }
}