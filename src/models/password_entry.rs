//! Defines a password entry
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordEntry {
    service: String,
    url: String,
    username: String,
    password: String,
    additional_nodes: Option<String>,
}
