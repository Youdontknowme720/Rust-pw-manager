use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        service: String,
        username: String,
        password: String,
    },

    Edit {
        service: String,

        #[arg(long)]
        username: Option<String>,

        #[arg(long)]
        password: Option<String>,
    },

    Find {
        password_id: String,
    },

    List,
}