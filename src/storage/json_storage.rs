use crate::models::password_entry::PasswordEntry;
use std::fs::File;
use std::io::BufReader;

pub struct JsonStorage {
    path: String,
}

impl JsonStorage {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }

    pub fn load(&self) -> Result<Vec<PasswordEntry>, std::io::Error> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);

        let entries: Vec<PasswordEntry> = serde_json::from_reader(reader)?;
        Ok(entries)
    }
    pub fn save(&self) -> Result<(), std::io::Error> {
        Ok(())
    }
}
