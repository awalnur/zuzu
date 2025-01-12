use std::collections::HashMap;
use std::fmt::Error;
use chrono::{DateTime, Utc};
use pasetors::claims::{Claims, ClaimsValidationRules};
use pasetors::footer::Footer;
use pasetors::keys::{Generate, SymmetricKey};
use pasetors::paserk::Id;
use pasetors::version4::V4;
use pasetors::{local, Local};
use pasetors::errors::ClaimValidationError;
use pasetors::errors::Error::ClaimValidation;
use pasetors::token::{TrustedToken, UntrustedToken};
use crate::utils::errors::AppError;

pub struct Claim{
    iss: String, // Provider ()
    jti: String, // token id
    aud: String, // audience, client or other
    nbf: DateTime<Utc>, //
    exp: DateTime<Utc>,
    iat: DateTime<Utc>,
    sub: String, // subject
}


pub trait Token {
    fn generate_token(&self) -> String;
    fn load_claims(&self, token: &str) -> Result<Claims, AppError>;
}



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

        let key = base64::decode("mL7h0mMOsML8DRNXfqGcc57j+AWnzTws9jgujQxq0xs=").expect("Failed to decode key");

        let sk = SymmetricKey::<V4>::from(&key).unwrap();
        let pid = Id::from(&sk);
        let mut footer = Footer::new();
        footer.key_id(&pid);

        let token = local::encrypt(&sk, &claims, Some(&footer), Some(b"implisit Assertion")).unwrap();
        token

    }

    fn load_claims(&self, token: &str) -> Result<Claims, AppError> {
        let key = base64::decode("mL7h0mMOsML8DRNXfqGcc57j+AWnzTws9jgujQxq0xs=").expect("Failed to decode key");


        let sk = SymmetricKey::<V4>::from(&key).unwrap();
        let pid = Id::from(&sk);
        println!("simetry key {:?}", &sk);
        let mut footer = Footer::new();
        footer.key_id(&pid);
        let validation_rules = ClaimsValidationRules::new();
        let untrusted_token = UntrustedToken::<Local, V4>::try_from(token).expect("Failed to get untrusted token");
        let claims: Result<Claims, AppError> = match local::decrypt(&sk, &untrusted_token, &validation_rules, None, Some(b"implisit Assertion")){
            Ok(claims) => claims.payload_claims().cloned().ok_or(AppError::Unauthorized("Unauthorized Access".to_string())),
            Err(e) => {
                if let ClaimValidation(e) = e {
                    if let ClaimValidationError::Exp = e {
                        return Err(AppError::Unauthorized("Token Expired".to_string()))
                    }
                    Err(AppError::Unauthorized("e".to_string()))
                } else {
                    Err(AppError::Unauthorized("Unauthorized Access".to_string()))
                }
            }
        };
        claims

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_token() {
        let claim = Claim {
            iss: "provider".to_string(),
            jti: "token_id".to_string(),
            aud: "audience".to_string(),
            nbf: Utc::now(),
            exp: Utc::now()+chrono::Duration::days(1),
            iat: Utc::now(),
            sub: "subject".to_string(),
        };
        let token = claim.generate_token();
        println!("token {:?}", token);
        assert_eq!(token.is_empty(), false);
    }

    #[test]
    fn test_load_claims() {
        let claim = Claim {
            iss: "provider".to_string(),
            jti: "token_id".to_string(),
            aud: "audience".to_string(),
            nbf: Utc::now(),
            exp: Utc::now()+chrono::Duration::days(1),
            iat: Utc::now(),
            sub: "subject".to_string(),
        };
        let token = claim.generate_token();
        let mut loaded_claim = claim.load_claims(&token).expect("Failed to load claims");
        println!("loaded claim {:?}", loaded_claim);
        assert_eq!(loaded_claim.get_claim("iss").expect("data"), "provider");
    }
}