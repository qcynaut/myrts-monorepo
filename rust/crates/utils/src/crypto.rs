/*
Copyright (c) 2023 Ade M Ramdani <qcynaut@gmail.com>

This software is proprietary and licensed to MyRTS under the terms of the Closed-Source Software License for Freelancers, which is available at https://dictionary.cambridge.org/us/dictionary/english/license.

MyRTS owns all right, title, and interest in and to the software, including all intellectual property rights therein.
MyRTS may use the software for any purpose, including commercial use.
MyRTS may modify the software, but only for their own internal use.
MyRTS may not distribute the software or any modified versions of the software to third parties.
MyRTS may not reverse engineer the software.
MyRTS may not create derivative works from the software.

MyRTS agrees to credit you as the developer of the software in all promotional materials and documentation for the software.

If MyRTS violates any of these terms, their license to use the software will automatically terminate.
*/

use sha3::{Digest, Sha3_256};

#[cfg(feature = "api")]
pub use self::api::*;

#[cfg(feature = "api")]
mod api {
    /// Bcrypt.
    /// Used to hash password.
    pub struct Bcrypt;

    impl Bcrypt {
        /// Hash the given password.
        pub fn hash(password: &str) -> Result<String, Box<dyn std::error::Error>> {
            let hashed = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
            Ok(hashed)
        }

        /// Verify the given password.
        pub fn verify(hashed: &str, password: &str) -> Result<bool, Box<dyn std::error::Error>> {
            Ok(bcrypt::verify(password, hashed)?)
        }
    }

    /// Jwt.
    /// Used to generate jwt token.
    #[derive(Clone)]
    pub struct Jwt {
        enc: jsonwebtoken::EncodingKey,
        dec: jsonwebtoken::DecodingKey,
    }

    /// JwtClaim.
    /// Used to generate jwt token.
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct JwtClaim {
        sub: i32,
        company: String,
        exp: u128,
    }

    impl Jwt {
        /// Create new Jwt.
        pub fn new(secret: &str) -> Self {
            let enc = jsonwebtoken::EncodingKey::from_secret(secret.as_bytes());
            let dec = jsonwebtoken::DecodingKey::from_secret(secret.as_bytes());
            Self { enc, dec }
        }

        /// Create a new jwt token by the given user id.
        pub fn create(&self, user_id: i32) -> Result<String, String> {
            let now = std::time::SystemTime::now();
            let exp = now + std::time::Duration::from_secs(60 * 60 * 24 * 30);
            let claim = JwtClaim {
                sub: user_id,
                company: "myrts".to_string(),
                exp: exp
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                    .map_err(|_| "invalid token".to_string())?
                    .as_millis(),
            };
            let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claim, &self.enc)
                .map_err(|_| "invalid token".to_string())?;
            Ok(token)
        }

        /// Check if token is valid.
        pub fn verify(&self, token: &str) -> Result<(i32, bool), String> {
            let claim = jsonwebtoken::decode::<JwtClaim>(token, &self.dec, &Default::default())
                .map_err(|_| "invalid token".to_string())?;
            Ok((
                claim.claims.sub,
                claim.claims.exp
                    < std::time::SystemTime::now()
                        .duration_since(std::time::SystemTime::UNIX_EPOCH)
                        .map_err(|_| "invalid token".to_string())?
                        .as_millis(),
            ))
        }
    }
}

/// Create random string.
pub fn random_string(length: usize) -> String {
    use rand::{distributions::Alphanumeric, Rng};
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

/// Create uuid.
pub fn uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Hash a bytes.
pub fn hash(bytes: &[u8]) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(bytes);
    let result = hasher.finalize();
    hex::encode(result)
}
