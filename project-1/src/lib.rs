use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Sets the value of a key to the specified value
    Set { key: String, value: String },
    /// Gets the value corresponding to a key
    Get { key: String },
    /// Removes a given key
    Rm { key: String },
}
