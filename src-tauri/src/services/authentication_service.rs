// TODO: Implement hashing and salting for passwords here
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

use argon2::password_hash::{self, rand_core::OsRng, SaltString};

pub struct AuthenticationService;
// TODO: Add JWT token generation and verification methods
impl AuthenticationService {
    pub fn new() -> Self {
        AuthenticationService
    }
    // TODO: Test hashing and verifying passwords
    pub fn hash_password(&self, password: &str) -> Result<String, password_hash::Error> {
        let argon2 = Argon2::default();

        let salt = SaltString::generate(&mut OsRng);
        match argon2.hash_password(password.as_bytes(), &salt) {
            Ok(hash) => Ok(hash.to_string()),
            Err(e) => Err(e),
        }
    }

    pub fn verify_password(
        &self,
        password: &str,
        hash: &str,
    ) -> Result<bool, password_hash::Error> {
        let parsed_hash = PasswordHash::new(hash)?;
        let argon2 = Argon2::default();

        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(password_hash::Error::Password) => Ok(false),
            Err(e) => Err(e),
        }
    }

    pub fn hash_and_verify(&self, password: &str) -> Result<String, password_hash::Error> {
        let hash = self.hash_password(password)?;
        self.verify_password(password, &hash)?;
        Ok(hash)
    }
}
