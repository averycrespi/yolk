use std::error;
use std::fmt;

/// Represents a general Yolk error.
#[derive(Debug, Clone)]
pub enum YolkError {
    NotImplemented,
    DuplicateImport { ident: String },
    ExistingImport { ident: String },
    ExistingFunction { ident: String },
    ExistingVariable { ident: String },
    DuplicateExport { ident: String },
    UndefinedExport { ident: String },
}

impl error::Error for YolkError {}

impl fmt::Display for YolkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            YolkError::NotImplemented => write!(f, "not implemented"),
            YolkError::DuplicateImport { ident } => {
                write!(f, "cannot import variable twice: {}", ident)
            }
            YolkError::ExistingImport { ident } => {
                write!(f, "cannot import existing variable: {}", ident)
            }
            YolkError::ExistingFunction { ident } => {
                write!(f, "cannot redefine existing function: {}", ident)
            }
            YolkError::ExistingVariable { ident } => {
                write!(f, "cannot assign to existing variable: {}", ident)
            }
            YolkError::DuplicateExport { ident } => {
                write!(f, "cannot export variable twice: {}", ident)
            }
            YolkError::UndefinedExport { ident } => {
                write!(f, "cannot export undefined variable: {}", ident)
            }
        }
    }
}
