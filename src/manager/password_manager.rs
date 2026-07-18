//! Defines a PasswordManager and its functions

use crate::models::password_entry::PasswordEntry;
use crate::storage::json_storage::JsonStorage;

pub struct PasswordManager {
    storage: JsonStorage,
}

impl PasswordManager {
    pub fn new(storage: JsonStorage) -> Self {
        Self { storage }
    }

    pub fn add(&self, entry: PasswordEntry) -> Result<(), String> {
        let load_content = self.storage.load();
        let mut entries = match load_content {
            Ok(e) => e,
            Err(e) => return Err(e.to_string()),
        };

        entries.push(entry);
        match self.storage.save(&entries) {
            Ok(_e) => println!("Successful"),
            Err(e) => return Err(e.to_string()),
        };

        Ok(())
    }
}
