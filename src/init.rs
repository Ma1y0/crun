use crate::{
    config::{Compiler, Config},
    Result,
};
use std::{env, fs};

fn get_project_name(name: Option<String>) -> Result<String> {
    let name = match name {
        Some(a) => a,
        None => {
            let path = env::current_dir()?;
            if let Some(wd) = path.file_name() {
                wd.to_string_lossy().to_string()
            } else {
                panic!("Failed to read current working directory");
            }
        }
    };

    Ok(name)
}

// Takes only `lib` arg because `bin` is the default
pub fn init_project(name: Option<String>, lib: bool) -> Result<()> {
    let name = get_project_name(name)?;
    let config = Config::new(
        name,
        "0.1.0".to_string(),
        None,
        Compiler::Clang,
        vec!["-Wall".to_string()],
    );

    if lib {
        todo!("Not implemented yet")
    } else {
        fs::create_dir("./build")?;
        fs::create_dir("./src")?;
        // fs::File::
    }

    config.write_to_file()?;

    Ok(())
}
