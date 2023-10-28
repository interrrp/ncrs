use clap::Parser;

use ncrs::Args;

fn main() {
    let args = Args::parse();
    println!("Connecting to {}", args.address);
}
