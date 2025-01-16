use std::string::ToString;
use argon2::{Argon2, ParamsBuilder, PasswordHasher, PasswordVerifier};
use argon2::Algorithm::Argon2d;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use crate::utils::errors::AppError;
use chrono::{DateTime, Utc};
use pasetors::claims::{Claims, ClaimsValidationRules};
use pasetors::errors::ClaimValidationError;
use pasetors::errors::Error::ClaimValidation;
use pasetors::footer::Footer;
use pasetors::keys::{SymmetricKey};
use pasetors::paserk::Id;
use pasetors::token::UntrustedToken;
use pasetors::version4::V4;
use pasetors::{local, Local};
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct Claim {
    pub(crate) iss: String,        // Provider ()
    pub(crate) jti: String,        // token id
    pub(crate) aud: String,        // audience, client or other
    pub(crate) nbf: DateTime<Utc>, //
    pub(crate) exp: DateTime<Utc>,
    pub(crate) iat: DateTime<Utc>,
    pub(crate) sub: String, // subject
}

pub struct Password {
    pub plain: String
}

pub trait Token {
    fn generate_token(&self) -> String;
    fn load_claims(&self, token: &str) -> Result<Claims, AppError>;
}

pub trait ArgonHash {
    fn hash_password(&self) -> String;
    fn verify_password(&self, hash: &str) -> bool;
}
static SECRET_KEY: Lazy<String> = Lazy::new(|| {
    std::env::var("TOKEN_SECRET_KEY").expect("TOKEN_SECRET_KEY must be set")
});
impl Token for Claim {

    fn generate_token(&self) -> String {
        let mut claims = Claims::new().unwrap();

        claims.issuer(&self.iss).unwrap();
        claims.subject(&self.sub).unwrap();
        claims.audience(&self.aud).unwrap();
        claims.expiration(&self.exp.to_rfc3339()).unwrap();
        claims.not_before(&self.nbf.to_rfc3339()).unwrap();
        claims.issued_at(&self.iat.to_rfc3339()).unwrap();
        claims.token_identifier(&self.jti).unwrap();

        let key = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, SECRET_KEY.as_str())
            .expect("Failed to decode key");

        let sk = SymmetricKey::<V4>::from(&key).unwrap();
        let pid = Id::from(&sk);
        let mut footer = Footer::new();
        footer.key_id(&pid);

        let token =
            local::encrypt(&sk, &claims, Some(&footer), Some(b"implisit Assertion")).unwrap();

        token
    }

    fn load_claims(&self, token: &str) -> Result<Claims, AppError> {
        let key = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, SECRET_KEY.as_str())
            .expect("Failed to decode key");

        let sk = SymmetricKey::<V4>::from(&key).unwrap();
        let pid = Id::from(&sk);
        let mut footer = Footer::new();
        footer.key_id(&pid);
        let validation_rules = ClaimsValidationRules::new();
        let untrusted_token = match UntrustedToken::<Local, V4>::try_from(token) {
            Ok(token) => token,
            Err(_) => return Err(AppError::Unauthorized("Invalid Token".to_string())),
        };
        let claims: Result<Claims, AppError> = match local::decrypt(
            &sk,
            &untrusted_token,
            &validation_rules,
            None,
            Some(b"implisit Assertion"),
        ) {
            Ok(claims) => claims
                .payload_claims()
                .cloned()
                .ok_or(AppError::Unauthorized("Unauthorized Access".to_string())),
            Err(e) => {
                if let ClaimValidation(e) = e {
                    match e {
                        ClaimValidationError::Nbf => {
                            return Err(AppError::Unauthorized(
                                "Failed to validate token time".to_string(),
                            ))
                        }
                        ClaimValidationError::Exp => {
                            return Err(AppError::Unauthorized("Token Expired".to_string()))
                        }
                        ClaimValidationError::Aud => {
                            return Err(AppError::Unauthorized(
                                "Failed to validate Audience ".to_string(),
                            ))
                        }
                        _ => return Err(AppError::Unauthorized("Unauthorized Access".to_string())),
                    }
                } else {
                    Err(AppError::Unauthorized(
                        "Error when claims token".to_string(),
                    ))
                }
            }
        };
        claims
    }
}

impl ArgonHash for Password {
    fn hash_password(&self) -> String {
        let salt = SaltString::generate(&mut OsRng);
        let params = ParamsBuilder::default().build().unwrap();

        let config = Argon2::new(Argon2d, argon2::Version::V0x13, params);
        let hash = config.hash_password(&self.plain.as_bytes(), &salt).unwrap();
        hash.to_string()
    }

    fn verify_password(&self, hash: &str) -> bool {

        let params = ParamsBuilder::default().build().unwrap();

        let config = Argon2::new(Argon2d, argon2::Version::V0x13, params);
        let parsed_hash = match argon2::PasswordHash::new(hash){
            Ok(hash) => hash,
            Err(_) => return false
        };
        config.verify_password(&self.plain.as_bytes(), &parsed_hash).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_generate_token() {
        env::set_var("TOKEN_SECRET_KEY", "mL7h0mMOsML8DRNXfqGcc57j+AWnzTws9jgujQxq0xs=");
        let claim = Claim {
            iss: "provider".to_string(),
            jti: "token_id".to_string(),
            aud: "audience".to_string(),
            nbf: Utc::now(),
            exp: Utc::now() + chrono::Duration::days(1),
            iat: Utc::now(),
            sub: "subject".to_string(),
        };
        let token = claim.generate_token();
        assert_eq!(token.is_empty(), false);
    }

    #[test]
    fn load_claims_with_expired_token() {
        env::set_var("TOKEN_SECRET_KEY", "mL7h0mMOsML8DRNXfqGcc57j+AWnzTws9jgujQxq0xs=");
        let claim = Claim {
            iss: "provider".to_string(),
            jti: "token_id".to_string(),
            aud: "audience".to_string(),
            nbf: Utc::now() - chrono::Duration::days(2),
            exp: Utc::now() - chrono::Duration::days(1),
            iat: Utc::now() - chrono::Duration::days(2),
            sub: "subject".to_string(),
        };
        let token = claim.generate_token();
        let result = claim.load_claims(&token);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Unauthorized: Token Expired");
    }

    #[test]
    fn load_claims_with_invalid_token() {
        env::set_var("TOKEN_SECRET_KEY", "mL7h0mMOsML8DRNXfqGcc57j+AWnzTws9jgujQxq0xs=");
        let claim = Claim {
            iss: "provider".to_string(),
            jti: "token_id".to_string(),
            aud: "audience".to_string(),
            nbf: Utc::now(),
            exp: Utc::now() + chrono::Duration::days(1),
            iat: Utc::now(),
            sub: "subject".to_string(),
        };
        let invalid_token = "invalid.token.string";
        let result = claim.load_claims(invalid_token);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Unauthorized: Invalid Token");
    }

    #[test]
    fn load_claims_with_not_before_token() {
        env::set_var("TOKEN_SECRET_KEY", "mL7h0mMOsML8DRNXfqGcc57j+AWnzTws9jgujQxq0xs=");
        let claim = Claim {
            iss: "provider".to_string(),
            jti: "token_id".to_string(),
            aud: "audience".to_string(),
            nbf: Utc::now() + chrono::Duration::days(1),
            exp: Utc::now() + chrono::Duration::days(2),
            iat: Utc::now(),
            sub: "subject".to_string(),
        };
        let token = claim.generate_token();
        let result = claim.load_claims(&token);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Unauthorized: Failed to validate token time");
    }

    #[test]
    fn load_claims_with_valid_token() {
        env::set_var("TOKEN_SECRET_KEY", "mL7h0mMOsML8DRNXfqGcc57j+AWnzTws9jgujQxq0xs=");
        let claim = Claim {
            iss: "provider".to_string(),
            jti: "token_id".to_string(),
            aud: "audience".to_string(),
            nbf: Utc::now(),
            exp: Utc::now() + chrono::Duration::days(1),
            iat: Utc::now(),
            sub: "subject".to_string(),
        };
        let token = claim.generate_token();
        let result = claim.load_claims(&token);
        assert!(result.is_ok());
    }
    #[test]
    fn hash_password_creates_valid_hash() {
        let pass = Password {
            plain: "".to_string()
            };
        let hash = pass.hash_password();
        assert!(!hash.is_empty());
    }

    #[test]
    fn verify_password_with_correct_password() {
        let pass = Password {
            plain: "password123".to_string()
        };
        let hash = pass.hash_password();
        let is_valid = pass.verify_password(&hash);
        assert!(is_valid);
    }

    #[test]
    fn verify_password_with_incorrect_password() {
        let pass = Password {
            plain: "password123".to_string()
        };
        let hash = pass.hash_password();
        let invalid_pass = Password {
            plain: "wrongpassword".to_string()
        };
        let is_valid = invalid_pass.verify_password(&hash);
        assert!(!is_valid);
    }

    #[test]
    fn verify_password_with_invalid_hash() {
        let password = Password{ plain:"password123".to_string()};
        let invalid_hash = "invalid_hash";
        let is_valid = password.verify_password(invalid_hash);
        assert!(!is_valid);
    }

}