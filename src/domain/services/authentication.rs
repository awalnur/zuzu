use std::collections::HashMap;
use actix_web::web;
use chrono::Utc;
use crate::utils::crypto::{Claim, Password, Token, ArgonHash};
use serde::{Serialize, Deserialize};
use crate::config::database::{DbPool};
use crate::domain::models::authentication::LoginRequest;
use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::repository::Repository;
use crate::utils::errors::AppError;

pub async  fn generate_token(
    pool: web::Data<DbPool>,
    payload: web::Json<LoginRequest>,
)-> Result<HashMap<String, String>, AppError> {
    let repo = UserRepository::new(pool);
    let user = repo.find_by_username(&payload.username).await;
    match user {
        Ok(user) => {
            let hasesd = String::from_utf8(user.1.unwrap().password_hash.expect("Error")).expect("Error");
            let password = Password{
                plain: payload.password.clone(),
            };
            println!("hash {:?}",password.hash_password());
            if !password.verify_password(&hasesd) {
                Err(AppError::Unauthorized("Invalid username or password".to_string()))
            }else{
                let token = Claim {
                    iss: "provider".to_string(),
                    jti: "token_id".to_string(),
                    aud: "audience".to_string(),
                    nbf: Utc::now(),
                    exp: Utc::now() + chrono::Duration::days(1),
                    iat: Utc::now(),
                    sub: "subject".to_string(),
                };
                let access_token = token.generate_token();
                let refresh_token = token.generate_token();
                let expires = Utc::now().timestamp() + 60 * 60 * 24 * 7; // 7 days
                let mut res =  HashMap::new();
                res.insert("access_token".to_string(), access_token);
                res.insert("refresh_token".to_string(), refresh_token);
                res.insert("expires".to_string(), expires.to_string());
                res.insert("token_type".to_string(), "Bearer".to_string());
                Ok(res)
            }
        }
        Err(e) =>Err(AppError::NotFound("User not found".to_string())),
        _ => Err(AppError::Unauthorized("Invalid username or password".to_string())),
    }
    // This function will be implemented in the next step
}