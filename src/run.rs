use crate::{compile, config::Config, Result};
use std::process::{Command, Stdio};

pub fn run(config: &Config) -> Result<()> {
    compile::compile(config);
    Command::new("./build/main")
        .stdin(Stdio::inherit())
        .output();

    Ok(())
}
