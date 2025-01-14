use actix_web::{web, Error};
use diesel::associations::HasTable;
use diesel::{
    debug_query, ExpressionMethods, JoinOnDsl, NullableExpressionMethods, QueryDsl,
    RunQueryDsl, SelectableHelper, TextExpressionMethods,
};

use crate::config::database::DbPool;
use crate::domain::models::user::{NewPasswordHash, NewUser, PasswordHash, User};
use crate::domain::repository::Repository;
use crate::infrastructure::database::schemas::schemas::accounts::dsl;
use crate::infrastructure::database::schemas::schemas::accounts::dsl::accounts;
use crate::infrastructure::database::schemas::schemas::password_hashes;
use crate::utils::crypto::{ArgonHash, Password};
use crate::utils::errors::AppError;
use uuid::Uuid;

pub struct UserRepository {
    pool: web::Data<DbPool>,
}

impl UserRepository {
    pub fn new(pool: web::Data<DbPool>) -> Self {
        UserRepository { pool }
    }
}
#[async_trait::async_trait]
impl Repository<User, Uuid, NewUser, NewUser, (User, Option<PasswordHash>)> for UserRepository {
    async fn find_all(
        &self,
        limit: Option<i16>,
        offsets: Option<i16>,
        search: Option<String>,
    ) -> Result<Vec<User>, Error> {
        let pool = self.pool.clone();
        let mut query = accounts
            .select(User::as_select())
            .into_boxed() // This boxes the query
            .limit(limit.unwrap_or(10) as i64)
            .offset(offsets.unwrap_or(0) as i64);

        let filtered = match search {
            Some(search) => query.filter(dsl::username.like(format!("%{}%", search.to_string()))),
            None => query,
        };

        println!("{:?}", debug_query(&filtered));
        let mut conn = pool
            .get()
            .map_err(|e| AppError::ServiceUnavailable(e.to_string()))?;

        let users = web::block(move || {
            let data = filtered
                .load::<User>(&mut conn)
                .expect("Error loading users");
            data
        })
        .await?;
        Ok(users)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<User, Error> {
        let query = accounts.filter(dsl::id.eq(id));
        let mut conn = self
            .pool
            .get()
            .map_err(|e| AppError::ServiceUnavailable(e.to_string()))?;
        let user = web::block(move || {
            query
                .first::<User>(&mut conn)
                .map_err(|e| AppError::NotFound(e.to_string()))
        })
        .await?;
        Ok(user?)
    }

    async fn find_by(&self, id: Uuid) -> Result<User, Error> {
        let query = accounts.filter(dsl::id.eq(id));
        let mut conn = self
            .pool
            .get()
            .map_err(|e| AppError::ServiceUnavailable(e.to_string()))?;
        let user = web::block(move || {
            query
                .first::<User>(&mut conn)
                .map_err(|e| AppError::NotFound(e.to_string()))
        })
        .await?;
        Ok(user?)
    }

    async fn find_by_email(&self, email: &str) -> Result<User, Error> {
        let query = accounts.filter(dsl::email.eq(email.to_string()));
        let mut conn = self
            .pool
            .get()
            .map_err(|e| AppError::ServiceUnavailable(e.to_string()))?;
        let user = web::block(move || {
            query
                .first::<User>(&mut conn)
                .map_err(|e| AppError::NotFound(e.to_string()))
        })
        .await?;
        Ok(user?)
    }
    async fn find_by_username(
        &self,
        username: &str,
    ) -> Result<(User, Option<PasswordHash>), Error> {
        let username = username.to_string();
        let query = accounts
            .left_outer_join(
                password_hashes::dsl::password_hashes::table()
                    .on(dsl::id.eq(password_hashes::dsl::user_id)),
            )
            .filter(dsl::username.eq(username))
            .select((
                User::as_select(),
                (
                    password_hashes::dsl::id.nullable(),
                    password_hashes::dsl::user_id.nullable(),
                    password_hashes::dsl::password_hash.nullable(),
                    password_hashes::dsl::salt.nullable(),
                    password_hashes::dsl::algorithm.nullable(),
                    password_hashes::dsl::is_temporary.nullable(),
                    password_hashes::dsl::expiry.nullable(),
                    password_hashes::dsl::last_change_at.nullable(),
                    password_hashes::dsl::created_at.nullable(),
                    password_hashes::dsl::updated_at.nullable(),
                )
                    .nullable(),
            ));
        let mut conn = self
            .pool
            .get()
            .map_err(|e| AppError::ServiceUnavailable(e.to_string()))?;
        let user = web::block(move || {
            query
                .first::<(User, Option<PasswordHash>)>(&mut conn)
                .map_err(|e| AppError::NotFound(e.to_string()))
        })
        .await?;
        Ok(user?)
    }

    async fn create(&self, data: NewUser) -> Result<User, Error> {
        let pool = self.pool.clone();
        let new_user = data.clone();
        let mut conn = pool
            .get()
            .map_err(|e| AppError::ServiceUnavailable(e.to_string()))?;
        let user = diesel::insert_into(accounts)
            .values(&new_user)
            .get_result::<User>(&mut conn);

        match user {
            Ok(user) => {
                let password = Password {
                    plain: data.username,
                };
                let hash = password.hash_password();
                let password_hash = NewPasswordHash {
                    user_id: user.id,
                    password_hash: hash.into_bytes(),
                    salt: "SALT".to_string().into_bytes(),
                    algorithm: "ALGO".to_string(),
                    is_temporary: false,
                    expiry: chrono::Utc::now().naive_utc() + chrono::Duration::days(265),
                };
                diesel::insert_into(password_hashes::dsl::password_hashes)
                    .values(&password_hash)
                    .execute(&mut conn)
                    .expect("Error inserting password hash");
                Ok(user)
            }
            Err(e) => {
                return Err(Error::from(AppError::NotFound(e.to_string())));
            }
        }
    }

    async fn update(&self, id: Uuid, entry: NewUser) -> Result<User, Error> {
        let pool = self.pool.clone();
        let mut conn = pool
            .get()
            .map_err(|e| AppError::ServiceUnavailable(e.to_string()))?;
        let user = diesel::update(accounts.filter(dsl::id.eq(id)));
        let user = web::block(move || {
            user
                .set(&entry)
                .get_result::<User>(&mut conn)
                .map_err(|e| AppError::NotFound(e.to_string()))
        })
        .await?;
        Ok(user?)
    }

    async fn delete(&self, id: Uuid) -> Result<(), Error> {
        let pool = self.pool.clone();
        let mut conn = pool
            .get()
            .map_err(|e| AppError::ServiceUnavailable(e.to_string()))?;
        let user = diesel::delete(accounts.filter(dsl::id.eq(id)));
        web::block(move || {
            user
                .execute(&mut conn)
                .map_err(|e| AppError::NotFound(e.to_string()))
        })
            .await?.expect("TODO: panic message");
        Ok(())
    }
}
