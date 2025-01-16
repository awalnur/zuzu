// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType, Debug, PartialEq)]
    #[diesel(postgres_type(name = "account_status_enum"))]
    pub struct AccountStatusEnum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType, Debug, PartialEq)]
    #[diesel(postgres_type(name = "token_type_enum"))]
    pub struct TokenTypeEnum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType, Debug, PartialEq)]
    #[diesel(postgres_type(name = "two_factor_method_enum"))]
    pub struct TwoFactorMethodEnum;
}

diesel::table! {
    account_roles (account_id, role_id) {
        account_id -> Uuid,
        role_id -> Uuid,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AccountStatusEnum;
    use super::sql_types::TwoFactorMethodEnum;

    accounts (id) {
        id -> Uuid,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        is_active -> Bool,
        is_verified -> Bool,
        phone_number -> Nullable<Varchar>,
        status -> AccountStatusEnum,
        last_login -> Nullable<Timestamptz>,
        two_factor_method -> TwoFactorMethodEnum,
        registration_date -> Timestamptz,
        preferred_language -> Varchar,
        login_attempts -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TokenTypeEnum;

    auth_tokens (jti) {
        jti -> Uuid,
        sub -> Uuid,
        expires -> Timestamptz,
        issued_at -> Timestamptz,
        ip_address -> Inet,
        device_info -> Json,
        is_active -> Bool,
        token_type -> TokenTypeEnum,
        #[max_length = 255]
        authorization_type -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    password_hashes (id) {
        id -> Uuid,
        user_id -> Uuid,
        password_hash -> Bytea,
        salt -> Bytea,
        #[max_length = 255]
        algorithm -> Varchar,
        is_temporary -> Bool,
        expiry -> Timestamptz,
        last_change_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    password_reset_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        token -> Varchar,
        expires_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    permissions (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    role_permissions (role_id, permission_id) {
        role_id -> Uuid,
        permission_id -> Uuid,
        read -> Bool,
        write -> Bool,
        update -> Bool,
        delete -> Bool,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    roles (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TwoFactorMethodEnum;

    two_factor_methods (id) {
        id -> Uuid,
        user_id -> Uuid,
        method -> TwoFactorMethodEnum,
        is_enabled -> Bool,
        backup_codes -> Nullable<Json>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(account_roles -> accounts (account_id));
diesel::joinable!(account_roles -> roles (role_id));
diesel::joinable!(auth_tokens -> accounts (sub));
diesel::joinable!(password_hashes -> accounts (user_id));
diesel::joinable!(password_reset_tokens -> accounts (user_id));
diesel::joinable!(role_permissions -> permissions (permission_id));
diesel::joinable!(role_permissions -> roles (role_id));
diesel::joinable!(two_factor_methods -> accounts (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    account_roles,
    accounts,
    auth_tokens,
    password_hashes,
    password_reset_tokens,
    permissions,
    role_permissions,
    roles,
    two_factor_methods,
);
