use crate::manager::password_manager::PasswordManager;
use crate::storage::json_storage::JsonStorage;

mod manager;
mod models;
mod storage;

fn main() {
    let s = JsonStorage::new("password.json");
    let manager: PasswordManager = PasswordManager::new(s);
    match manager.edit("Uber".to_string(), None, Some("IKnow".to_string())) {
        Ok(_) => println!("Successful edited the password"),
        Err(e) => println!("Error -> {}", e),
    };
}
