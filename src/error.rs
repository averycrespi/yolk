use std::error;
use std::fmt;

/// Represents a general Yolk error.
#[derive(Debug, Clone)]
pub enum YolkError {
    NotImplemented,
    DuplicateImport(String),
    ExistingImport(String),
    ExistingFunction(String),
    ExistingVariable(String),
    UndefinedVariable(String),
    DuplicateExport(String),
}

impl error::Error for YolkError {}

impl fmt::Display for YolkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            YolkError::NotImplemented => write!(f, "not implemented"),
            YolkError::DuplicateImport(ident) => write!(f, "duplicate import: {}", ident),
            YolkError::ExistingImport(ident) => {
                write!(f, "cannot import existing variable: {}", ident)
            }
            YolkError::ExistingFunction(ident) => {
                write!(f, "cannot redefine existing function: {}", ident)
            }
            YolkError::ExistingVariable(ident) => {
                write!(f, "cannot reassign existing variable: {}", ident)
            }
            YolkError::UndefinedVariable(ident) => write!(f, "undefined variable: {}", ident),
            YolkError::DuplicateExport(ident) => write!(f, "duplicate export: {}", ident),
        }
    }
}
