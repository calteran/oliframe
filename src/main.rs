mod parser;

use clap::Parser;
use parser::Args;

fn main() {
    let args = Args::parse();

    println!("{:?}", args);
}
