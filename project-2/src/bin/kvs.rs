use clap::Parser;
use kvs::{Args, Command, KvStore, Result};

fn main() -> Result<()> {
    let args = Args::parse();
    let directory = std::env::current_dir().unwrap();
    let mut store = KvStore::open(&directory)?;

    let result = match args.command {
        Command::Set { key, value } => store.set(&key, &value),
        Command::Get { .. } => panic!("unimplemented"),
        Command::Rm { key } => store.remove(&key),
    } ;

    if let Err(e) = result {
        eprintln!("{e}");
        std::process::exit(1);
    }

    Ok(())
}
