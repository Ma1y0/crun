use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initilize a new project inside current directory
    Init {
        /// Creates new library
        #[arg(long)]
        lib: bool,

        // Creates new binary project (the default behavior)
        #[arg(long)]
        bin: bool,
    },
    /// Creates a new project
    New {
        /// Path
        path: String,
        /// Creates new library
        #[arg(long)]
        lib: bool,

        // Creates new binary project (the default behavior)
        #[arg(long)]
        bin: bool,
    },
    /// Compiles and runs the project
    Run,
    /// Compiles the project
    Build,
    /// Tests the project
    Test,
}
