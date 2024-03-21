use crate::{config::Config, CrunError, Result};
use std::process::Command;

pub fn compile(config: &Config) -> Result<()> {
    let output = Command::new(config.CC.to_string())
        .args(["./src/main.c", "-o", "./build/main"])
        .args(config.CFLAGS.as_slice())
        .output()?;

    if !output.status.success() {
        return Err(Box::new(CrunError::CompilerError(
            String::from_utf8_lossy(output.stderr.as_slice()).to_string(),
        )));
    }

    Ok(())
}
