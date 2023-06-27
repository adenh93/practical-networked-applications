use clap::Parser;
use kvs::{Args, Command, KvStore, Result};

fn main() -> Result<()> {
    let args = Args::parse();

    if let Err(e) = run(args) {
        eprintln!("{e}");
        std::process::exit(1);
    }

    Ok(())
}

fn run(args: Args) -> Result<()> {
    let directory = std::env::current_dir().unwrap();
    let mut store = KvStore::open(&directory)?;

    match args.command {
        Command::Set { key, value } => store.set(&key, &value)?,
        Command::Rm { key } => store.remove(&key)?,
        Command::Get { key } => match store.get(&key)? {
            Some(value) => println!("{value}"),
            None => println!("Key not found"),
        },
    };

    Ok(())
}
