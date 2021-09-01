use totp_rs::{TOTP, Algorithm};
use super::common::AUTH_KEY_LENGTH;
use crate::utils::current_time;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct AuthKey(pub [u8; AUTH_KEY_LENGTH]);

impl AuthKey {
    pub fn from_buffer(key_buffer: [u8; AUTH_KEY_LENGTH]) -> Option<AuthKey> {
        Some(AuthKey(key_buffer))
    }

    pub fn to_string(&self) -> String {
        let mut buffer = [0u8; AUTH_KEY_LENGTH];
        let bytes_out = binascii::bin2hex(&self.0, &mut buffer).ok().unwrap();
        String::from(std::str::from_utf8(bytes_out).unwrap())
    }
}

impl std::str::FromStr for AuthKey {
    type Err = binascii::ConvertError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = Self { 0: [0u8; AUTH_KEY_LENGTH] };
        if s.len() != AUTH_KEY_LENGTH {
            return Err(binascii::ConvertError::InvalidInputLength);
        }
        binascii::hex2bin(s.as_bytes(), &mut i.0)?;
        Ok(i)
    }
}

pub struct KeyManager {
    totp: TOTP<String>,
}

impl KeyManager {
    pub fn new(secret: String) -> KeyManager {

        let totp = TOTP::new(
            Algorithm::SHA1,
            AUTH_KEY_LENGTH,
            1,
            30,
            secret,
        );

        KeyManager {
            totp
        }
    }

    pub fn generate_auth_key(&self, seconds_valid: u64) -> AuthKey {
        let auth_key = self.totp.generate(seconds_valid);

        // is always valid
        AuthKey::from_str(&auth_key).unwrap()
    }

    pub fn verify_auth_key(&self, auth_key: &AuthKey) -> bool {
        let current_time = current_time();

        self.totp.check(&auth_key.to_string(), current_time)
    }
}
