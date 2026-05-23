use domain_identity::domain::entities::role::Role;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Subject prefix marking that a token was issued for an API key (not a user).
pub const API_KEY_SUBJECT_PREFIX: &str = "apikey:";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub tenant_id: Uuid,
    pub role: Role,
    pub jti: Uuid,
    pub iss: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug)]
pub struct SignedToken {
    pub value: String,
    pub jti: Uuid,
    pub expires_at: time::OffsetDateTime,
}

/// Sign a short-lived access token (typically 15 min).
pub fn sign_access_token(
    user_id: Uuid,
    tenant_id: Uuid,
    role: Role,
    secret: &str,
    issuer: &str,
    ttl_minutes: u32,
) -> Result<SignedToken, jsonwebtoken::errors::Error> {
    sign(user_id, tenant_id, role, secret, issuer, ttl_minutes as i64 * 60)
}

/// Sign a long-lived refresh token (typically 30 days). The returned JTI must be
/// persisted on the server side along with the hash of `value` so it can be
/// revoked.
pub fn sign_refresh_token(
    user_id: Uuid,
    tenant_id: Uuid,
    role: Role,
    secret: &str,
    issuer: &str,
    ttl_days: u32,
) -> Result<SignedToken, jsonwebtoken::errors::Error> {
    sign(
        user_id,
        tenant_id,
        role,
        secret,
        issuer,
        ttl_days as i64 * 24 * 3600,
    )
}

fn sign(
    sub: Uuid,
    tenant_id: Uuid,
    role: Role,
    secret: &str,
    issuer: &str,
    ttl_seconds: i64,
) -> Result<SignedToken, jsonwebtoken::errors::Error> {
    let now = time::OffsetDateTime::now_utc();
    let expires_at = now + time::Duration::seconds(ttl_seconds);
    let jti = Uuid::new_v4();

    let claims = Claims {
        sub,
        tenant_id,
        role,
        jti,
        iss: issuer.to_string(),
        iat: now.unix_timestamp() as usize,
        exp: expires_at.unix_timestamp() as usize,
    };

    let value = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(SignedToken {
        value,
        jti,
        expires_at,
    })
}

pub fn verify_jwt(
    token: &str,
    secret: &str,
    issuer: &str,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let mut validation = Validation::default();
    validation.set_issuer(&[issuer]);
    validation.leeway = 5;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;

    Ok(token_data.claims)
}
