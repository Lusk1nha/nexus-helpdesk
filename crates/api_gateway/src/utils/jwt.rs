use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// O que vai dentro do Token. Isso trafega no Frontend (NÃO coloque senhas aqui)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,       // user_id
    pub tenant_id: Uuid, // Importante para o Multi-Tenancy!
    pub role: String,
    pub exp: usize, // Data de expiração
}

pub fn sign_jwt(
    user_id: Uuid,
    tenant_id: Uuid,
    role: &str,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24)) // Token dura 24 horas
        .expect("Timestamp inválido")
        .timestamp();

    let claims = Claims {
        sub: user_id,
        tenant_id,
        role: role.to_string(),
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}
