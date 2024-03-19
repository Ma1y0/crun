use crate::Result;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Config {
    CC: Compiler,
    CFLAGS: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Compiler {
    #[serde(rename = "clang")]
    Clang,
    #[serde(rename = "gcc")]
    GCC,
}

impl Config {
    pub fn to_json(&self) -> Result<String> {
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
            CC: Compiler::Clang,
            CFLAGS: vec!["-Wall".to_string()],
        };

        assert_eq!(
            a.to_json().unwrap(),
            "{\"CC\":\"clang\",\"CFLAGS\":[\"-Wall\"]}".to_string()
        );
    }

    #[test]
    fn json_to_config() {
        let json = "{\"CC\":\"clang\",\"CFLAGS\":[\"-Wall\"]}";
        let config = Config {
            CC: Compiler::Clang,
            CFLAGS: vec!["-Wall".to_string()],
        };

        assert_eq!(Config::from_json(json).unwrap(), config);

        // Fails on unsupported compiler
        assert!(Config::from_json("{\"CC\": \"tcc\",\"CFLAGS\":[\"-Wall\"]}").is_err());
    }
}
