use std::fs;

use crate::Result;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Config {
    project_name: String,
    version: String,
    description: Option<String>,
    pub CC: Compiler,
    pub CFLAGS: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Compiler {
    #[serde(rename = "clang")]
    Clang,
    #[serde(rename = "gcc")]
    GCC,
}

impl ToString for Compiler {
    fn to_string(&self) -> String {
        match self {
            Self::Clang => "clang".to_string(),
            Self::GCC => "gcc".to_string(),
        }
    }
}

impl Config {
    #[allow(non_snake_case)]
    pub fn new(
        project_name: String, version: String, description: Option<String>, CC: Compiler,
        CFLAGS: Vec<String>,
    ) -> Self {
        Self {
            project_name,
            version,
            description,
            CC,
            CFLAGS,
        }
    }

    pub fn write_to_file(&self) -> Result<()> {
        let json = self.to_json_pretty()?;
        fs::write("crun.json", json)?;

        Ok(())
    }

    pub fn read_from_file() -> Result<Self> {
        let file = fs::read_to_string("crun.json")?;
        Self::from_json(&file)
    }

    fn to_json_pretty(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self)?)
    }

    #[allow(unused)]
    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }

    fn from_json(s: &str) -> Result<Self> {
        let config: Config = serde_json::from_str(s)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_to_json() {
        let a = Config {
            project_name: "project".to_string(),
            version: "0.1.0".to_string(),
            description: None,
            CC: Compiler::Clang,
            CFLAGS: vec!["-Wall".to_string()],
        };

        println!("{}", a.to_json().unwrap());

        assert_eq!(
            a.to_json().unwrap(),
            "{\"project_name\":\"project\",\"version\":\"0.1.0\",\"description\":null,\"CC\":\"clang\",\"CFLAGS\":[\"-Wall\"]}".to_string()
        );
    }

    #[test]
    fn json_to_config() {
        let json = "{\"project_name\":\"project\",\"version\":\"0.1.0\",\"description\":null,\"CC\":\"clang\",\"CFLAGS\":[\"-Wall\"]}";
        let config = Config {
            project_name: "project".to_string(),
            version: "0.1.0".to_string(),
            description: None,
            CC: Compiler::Clang,
            CFLAGS: vec!["-Wall".to_string()],
        };

        assert_eq!(Config::from_json(json).unwrap(), config);

        // Fails on unsupported compiler
        assert!(Config::from_json("{\"CC\": \"tcc\",\"CFLAGS\":[\"-Wall\"]}").is_err());
    }
}
