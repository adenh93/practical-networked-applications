use clap::Parser;
use kvs::cli::Cli;

fn main() {
    let args = Cli::parse();

    match args {
        Cli::Set(_) => unimplemented!(),
        Cli::Rm(_) => unimplemented!(),
        Cli::Get(_) => unimplemented!(),
    };
}
