use crate::ast::InfixOp;

use std::error;
use std::fmt;

/// Represents a general Yolk error.
#[derive(Debug, Clone)]
pub enum YolkError {
    NotImplemented,
    DuplicateImport(String),
    DuplicateExport(String),
    ExistingImport(String),
    ExistingFunction(String),
    ExistingVariable(String),
    UndefinedVariable(String),
    UndefinedFunction(String),
    MismatchingArrays(InfixOp),
    NestedArrays,
}

impl error::Error for YolkError {}

impl fmt::Display for YolkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            YolkError::NotImplemented => write!(f, "not implemented"),
            YolkError::DuplicateImport(ident) => write!(f, "duplicate import: {}", ident),
            YolkError::DuplicateExport(ident) => write!(f, "duplicate export: {}", ident),
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
            YolkError::UndefinedFunction(ident) => write!(f, "undefined function: {}", ident),
            YolkError::MismatchingArrays(op) => {
                write!(f, "cannot apply operation to mismatching arrays: {:?}", op)
            }
            YolkError::NestedArrays => write!(f, "cannot nest arrays"),
        }
    }
}
