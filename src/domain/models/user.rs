
use crate::infrastructure::database::schemas::schemas::{password_hashes, accounts};
use crate::infrastructure::database::schemas::{ schemas::sql_types };
use diesel::{Queryable, Identifiable, Insertable, Selectable, AsChangeset};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Selectable, Queryable, Identifiable, Serialize, Clone)]
#[diesel(table_name = accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub is_active: bool,
    pub is_verified: bool,
    pub phone_number: Option<String>,
    pub status: AccountStatusEnum,
    pub last_login: Option<chrono::NaiveDateTime>,
    pub two_factor_method: TwoFactorMethodEnum,
    pub registration_date:chrono::NaiveDateTime,
    pub preferred_language: String,
    pub login_attempts: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable, Clone,AsChangeset)]
#[diesel(table_name = accounts)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub phone_number: String,
    pub is_active: bool,
    pub is_verified: bool,
    pub registration_date: chrono::NaiveDateTime,
    pub last_login: chrono::NaiveDateTime,
    pub two_factor_method: TwoFactorMethodEnum,
    pub preferred_language: Option<String>,
    pub status: AccountStatusEnum,
}

#[derive(Debug,  Serialize, Deserialize, DbEnum, Clone)]
#[ExistingTypePath = "sql_types::TwoFactorMethodEnum"]
pub enum TwoFactorMethodEnum {
    None,
    Email,
    Whatsapp,
    Totp,
    Sms,
}


#[derive(Debug,  Serialize, Deserialize, DbEnum, Clone)]
#[ExistingTypePath = "sql_types::AccountStatusEnum"]
pub enum AccountStatusEnum {
    Active,
    Locked,
    Suspended,
    Deleted,
}


#[derive(Debug, Deserialize, Selectable, Queryable, Identifiable, Serialize, Clone)]
#[diesel(table_name = password_hashes)]
#[diesel(belongs_to(User))]
pub struct PasswordHash{
    pub id: Option<uuid::Uuid>,
    pub user_id: Option<uuid::Uuid>,
    pub password_hash: Option<Vec<u8>>,
    pub salt: Option<Vec<u8>>,
    pub algorithm: Option<String>,
    pub is_temporary: Option<bool>,
    pub last_change_at: Option<chrono::NaiveDateTime>,
    pub expiry: Option<chrono::NaiveDateTime>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = password_hashes)]
pub struct NewPasswordHash {
    pub user_id: uuid::Uuid,
    pub password_hash: Vec<u8>,
    pub salt: Vec<u8>,
    pub algorithm: String,
    pub is_temporary: bool,
    pub expiry: chrono::NaiveDateTime,
}