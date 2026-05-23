use base64::Engine;
use rand::RngCore;
use sha2::{Digest, Sha256};

/// SHA-256 hash of a secret, hex-encoded. Used to store refresh tokens and
/// API keys so the database never holds the original value.
pub fn sha256_hex(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    hex::encode(hasher.finalize())
}

/// Generates a random API key in the form `nxk_<prefix>.<secret>`.
/// - `prefix` is 8 chars from the random bytes (public, stored as `key_prefix`)
/// - `secret` is the URL-safe base64 of 32 more bytes
///
/// Returns `(full_key, key_prefix, key_hash)`. The full key is shown to the
/// user **only once**; only `key_prefix` and `key_hash` are persisted.
pub fn generate_api_key() -> (String, String, String) {
    let mut buf = [0u8; 24];
    rand::thread_rng().fill_bytes(&mut buf);
    let prefix: String = buf[..6]
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
        .chars()
        .take(8)
        .collect();

    let mut secret_buf = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut secret_buf);
    let secret = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(secret_buf);

    let full = format!("nxk_{}.{}", prefix, secret);
    let hash = sha256_hex(&full);
    (full, prefix, hash)
}
