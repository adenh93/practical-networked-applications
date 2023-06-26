use clap::Parser;
use kvs::{Args, Command, Result};

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Set { .. } => panic!("unimplemented"),
        Command::Get { .. } => panic!("unimplemented"),
        Command::Rm { .. } => panic!("unimplemented"),
    };
}
