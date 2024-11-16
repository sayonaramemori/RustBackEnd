use chrono::{Utc,TimeDelta};
use serde::de::DeserializeOwned;
use serde::{Serialize,Deserialize};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::sync::Arc;
use lazy_static::lazy_static;

const SECRETE: &[u8] = b"sbjavawocnm6972";

pub struct TokenManager{
    key: Option<Arc<Hmac<Sha256>>>,
}

#[derive(Deserialize,Serialize)]
struct ClaimWrapper<T>{
    claim: T,
    expire_time: i64,
}

impl TokenManager
{
    pub fn new() ->TokenManager
    {
        lazy_static!{
            static ref KEY: Arc<Hmac<Sha256>> = Arc::new(Hmac::new_from_slice(SECRETE).unwrap());
        }
        TokenManager{
            key: Some(KEY.clone()),
        }
    }

    pub fn generate_token<T>(&self, claim: &T) -> String
    where T: DeserializeOwned + Serialize
    {
        let key = self.key.as_ref().unwrap();
        let claim_wrapper = ClaimWrapper{ claim , expire_time: 0};
        let token = claim_wrapper.sign_with_key(key.as_ref()).unwrap();
        token
    }

    pub fn generate_token_with_time<T>(&self, claim: &T, time: TimeDelta) -> String
    where T: DeserializeOwned + Serialize
    {
        let key = self.key.as_ref().unwrap();
        let claim_wrapper = ClaimWrapper{ claim , expire_time: Utc::now().checked_add_signed(time).unwrap().timestamp()};
        let token = claim_wrapper.sign_with_key(key.as_ref()).unwrap();
        token
    }

    pub fn verify_time<T>(&self, token: &str) -> bool
    where T: DeserializeOwned + Serialize
    {
        let key = self.key.as_ref().unwrap();
        if let Ok(claim) = VerifyWithKey::<ClaimWrapper<T>>::verify_with_key(token, key.as_ref()){
            if claim.expire_time == 0 || claim.expire_time > Utc::now().timestamp() {
                return true;
            }
        }
        false
    }

    pub fn unravel_with_time_check<T>(&self, token: &str)-> Option<T>
    where T: DeserializeOwned + Serialize
    {
        let key = self.key.as_ref().unwrap();
        if let Ok(claim) = VerifyWithKey::<ClaimWrapper<T>>::verify_with_key(token, key.as_ref()){
            if claim.expire_time == 0 || claim.expire_time > Utc::now().timestamp() {
                return Some(claim.claim);
            }
        }
        None
    }

    pub fn unravel_as<T>(&self, token: &str)-> Option<T>
    where T: DeserializeOwned + Serialize
    {
        let key = self.key.as_ref().unwrap();
        match VerifyWithKey::<ClaimWrapper<T>>::verify_with_key(token, key.as_ref()){
            Ok(claim) => Some(claim.claim),
            _ => None,
        }
    }
}