use clap::Parser;
use kvs::Args;

fn main() {
    let args = Args::parse();
    println!("{args:?}");
}
