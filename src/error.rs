#[derive(Debug)]
pub enum CrunError {
    ParseError,
    CompilerError(String),
    Other(String),
}

impl std::fmt::Display for CrunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError => write!(f, "Failed to parse the config file"),
            Self::CompilerError(a) => write!(f, "Compiler Error occured: {}", a),
            Self::Other(a) => write!(f, "Error occured {}", a),
        }
    }
}

impl std::error::Error for CrunError {}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
