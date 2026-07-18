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

    pub fn add(&self, entry: PasswordEntry) -> Result<(), Box<dyn std::error::Error>> {
        let mut entries = self.storage.load()?;

        entries.push(entry);
        self.storage.save(&entries)?;

        Ok(())
    }

    pub fn find(&self, password_id: String) -> Result<PasswordEntry, String> {
        let load_content = self.storage.load();
        let entries = match load_content {
            Ok(e) => e,
            Err(e) => return Err(e.to_string()),
        };

        for ent in entries {
            if *ent.service == password_id {
                return Ok(ent);
            }
        }
        Err("Password not found".to_string())
    }

    pub fn edit(&self, password_id: String, new_username: Option<String>, new_password: Option<String> ) -> Result<(), Box<dyn std::error::Error>> {
        let mut entries = self.storage.load()?;
        for entry in &mut entries {
            if entry.service == password_id {

                if let Some(username) = new_username {
                    entry.username = username;
                }

                if let Some(password) = new_password {
                    entry.password = password;
                }

                break;
            }
        }

        self.storage.save(&entries)?;
        Ok(())
    }
}
