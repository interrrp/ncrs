use clap::Parser;

use ncrs::{run_client, run_server, Args};

fn main() {
    let args = Args::parse();

    if args.server {
        run_server(&args.address)
    } else {
        run_client(&args.address)
    }
}
