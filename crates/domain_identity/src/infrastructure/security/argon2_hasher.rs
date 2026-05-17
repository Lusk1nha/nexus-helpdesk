use crate::domain::error::DomainError;
use crate::domain::ports::PasswordHasher;
use argon2::{
    Argon2,
    password_hash::{
        PasswordHash, PasswordHasher as HasherTrait, PasswordVerifier, SaltString, rand_core::OsRng,
    },
};

pub struct Argon2Hasher;

impl Argon2Hasher {
    pub fn new() -> Self {
        Self
    }
}

impl PasswordHasher for Argon2Hasher {
    fn hash(&self, plain_password: &str) -> Result<String, DomainError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(plain_password.as_bytes(), &salt)
            .map_err(|e| DomainError::SecurityError(format!("Falha ao gerar hash: {}", e)))?;

        Ok(password_hash.to_string())
    }

    fn verify(&self, plain_password: &str, hash: &str) -> Result<bool, DomainError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| DomainError::SecurityError(format!("Hash malformado no banco: {}", e)))?;

        let is_valid = Argon2::default()
            .verify_password(plain_password.as_bytes(), &parsed_hash)
            .is_ok();

        Ok(is_valid)
    }
}
