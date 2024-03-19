use clap::Parser;
use crun::{Args, Commands};

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Init { .. } => println!("Die"),
        _ => todo!("AAA"),
    }
}
