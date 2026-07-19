use crate::manager::password_manager::PasswordManager;
use crate::models::password_entry::PasswordEntry;
use crate::parser::cli::{Cli, Commands};
use crate::storage::json_storage::JsonStorage;
use clap::Parser;

mod manager;
mod models;
pub mod parser;
mod storage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let storage: JsonStorage = JsonStorage::new("password.json");
    let manager = PasswordManager::new(storage);

    match cli.command {
        Commands::Add {
            service,
            username,
            password,
        } => {
            let new_entry: PasswordEntry = PasswordEntry {
                service,
                url: "New".to_string(),
                username,
                password,
                additional_nodes: None,
            };
            manager.add(new_entry)?;
        }

        Commands::Edit {
            service,
            username,
            password,
        } => {
            manager.edit(service, username, password)?;
        }

        Commands::Find {
            password_id
        } => {
            manager.find(password_id)?;
        }

        Commands::List => {
            manager.display()?;
        }
    }

    Ok(())
}
