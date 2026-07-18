use crate::manager::password_manager::PasswordManager;
use crate::storage::json_storage::JsonStorage;

mod manager;
mod models;
mod storage;

fn main() {
    let s = JsonStorage::new("password.json");
    let manager: PasswordManager = PasswordManager::new(s);
    match manager.find("Uber".to_string()) {
        Ok(entry) => println!("Found -> {:?}", entry),
        Err(_) => println!("Couldn't find entry"),
    };
}
