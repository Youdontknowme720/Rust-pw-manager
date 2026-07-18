use crate::storage::json_storage::JsonStorage;

mod models;
mod storage;
mod manager;

fn main() {
    let s = JsonStorage::new("password.json");
    let res = s.load();
    match res {
        Ok(c) => println!("Content {:?}", c),
        Err(e) => println!("Error -> {}", e)
    }
}
