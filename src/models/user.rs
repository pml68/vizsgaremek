use serde::{Deserialize, Serialize};
use uuid::fmt::Hyphenated;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserModel {
    id: Hyphenated,
    disabled: bool,
    username: String,
    email_address: String,
    first_name: String,
    last_name: String,
    pass: Vec<u8>,
    totp_secret: Vec<u8>,
}
