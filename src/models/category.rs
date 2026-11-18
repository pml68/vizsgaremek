use serde::{Deserialize, Serialize};
use uuid::fmt::Hyphenated;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryModel {
    id: Hyphenated,
    user_id: Option<Hyphenated>,
    name: String,
}
