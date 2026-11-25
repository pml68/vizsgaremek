use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::fmt::Hyphenated;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepeatingTransactionModel {
    id: Hyphenated,
    user_id: Hyphenated,
    title: String,
    category_id: Hyphenated,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    repeats_at: DateTime<Utc>,
    value: i32,
    description: Option<String>,
}
