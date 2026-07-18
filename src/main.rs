use crate::manager::password_manager::PasswordManager;
use crate::models::password_entry::PasswordEntry;
use crate::storage::json_storage::JsonStorage;

mod models;
mod storage;
mod manager;

fn main() {
    let s = JsonStorage::new("password.json");
    let manager: PasswordManager = PasswordManager::new(s);
    let new_entry: PasswordEntry = PasswordEntry{
        service: "Uber".to_string(),
        url: "asjkdf".to_string(),
        username: "Ayanokouji".to_string(),
        password: "Whoknows".to_string(),
        additional_nodes: Some("Ayanokouji".to_string()),
    };
    match manager.add(new_entry){
        Ok(_) => println!("Saved Password"),
        Err(e) => println!("{}", e)
    };
}
