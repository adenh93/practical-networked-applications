use clap::Parser;
use kvs::{Args, Command};

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Set { .. } => panic!("unimplemented"),
        Command::Get { .. } => panic!("unimplemented"),
        Command::Rm { .. } => panic!("unimplemented"),
    };
}
