use serde::Serialize;
use time::OffsetDateTime;

/// Standard JSON envelope for all successful API responses.
///
/// ```json
/// {
///   "data": { ... },
///   "meta": { "timestamp": "2026-05-19T12:00:00Z" }
/// }
/// ```
#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub data: T,
    pub meta: Meta,
}

#[derive(Serialize)]
pub struct Meta {
    pub timestamp: OffsetDateTime,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            data,
            meta: Meta {
                timestamp: OffsetDateTime::now_utc(),
            },
        }
    }
}
