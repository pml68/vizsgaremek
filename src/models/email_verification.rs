use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::fmt::Hyphenated;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmailVerificationModel {
    token: Vec<u8>,
    user_id: Hyphenated,
    expires_at: DateTime<Utc>,
}
