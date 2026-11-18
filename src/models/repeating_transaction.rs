use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::fmt::Hyphenated;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepeatingTransactionModel {
    id: Hyphenated,
    original_id: Hyphenated,
    repeats_at: DateTime<Utc>,
}
