//! Defines a password entry
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordEntry {
    pub service: String,
    pub url: String,
    pub username: String,
    pub password: String,
    pub additional_nodes: Option<String>,
}
