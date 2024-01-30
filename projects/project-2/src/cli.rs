use clap::{Args, Parser};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub enum Cli {
    /// Set the value of a string key to a string.
    Set(SetArgs),
    // Get the string value of a given string key.
    Get(GetArgs),
    /// Remove a given key.
    Rm(RmArgs),
}

#[derive(Debug, Args)]
pub struct SetArgs {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Args)]
pub struct GetArgs {
    pub key: String,
}

#[derive(Debug, Args)]
pub struct RmArgs {
    pub key: String,
}
