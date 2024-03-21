use clap::Parser;
use crun::{compile, config::Config, init, run, Args, Commands, Result};

fn main() -> Result<()> {
    let args = Args::parse();
    let config = Config::read_from_file()?;

    let res = match args.command {
        Commands::Init { name, lib, .. } => init::init_project(name, lib),
        Commands::Build => compile::compile(&config),
        Commands::Run => run::run(&config),
        _ => todo!("AAA"),
    };

    match res {
        Ok(()) => println!("OK"),
        Err(e) => println!("{}", e),
    };

    Ok(())
}
