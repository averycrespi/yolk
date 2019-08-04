use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum YolkError {
    ExistingImport { ident: String },
}

impl error::Error for YolkError {}

impl fmt::Display for YolkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            YolkError::ExistingImport { ident } => {
                write!(f, "cannot import existing variable: {}", ident)
            }
        }
    }
}
