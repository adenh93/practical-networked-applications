use anyhow::Result;
use clap::Parser;
use project_2::{Cli, KvStore, KvsError};

fn main() -> Result<()> {
    let args = Cli::parse();
    let path = std::env::current_dir()?;
    let mut store = KvStore::open(path)?;

    match args {
        Cli::Set(args) => store.set(&args.key, &args.value)?,
        Cli::Rm(args) => store.remove(&args.key)?,
        Cli::Get(args) => {
            let value = store.get(&args.key)?.ok_or(KvsError::KeyNotFound)?;
            println!("{value}");
        }
    };

    Ok(())
}
