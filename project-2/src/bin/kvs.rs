use std::path::Path;

use clap::Parser;
use kvs::{Args, Command, Result, KvStore};

fn main() -> Result<()> {
    let args = Args::parse();
    let mut store = KvStore::open(Path::new("log.db"))?;

    match args.command {
        Command::Set { key, value } => store.set(&key, &value)?,
        Command::Get { .. } => panic!("unimplemented"),
        Command::Rm { .. } => panic!("unimplemented"),
    };

    Ok(())
}
