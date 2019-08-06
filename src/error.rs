use crate::ast::InfixOp;

use std::error;
use std::fmt;

/// Represents a general Yolk error.
#[derive(Debug, Clone)]
pub enum YolkError {
    ConflictingVariable(String),
    DuplicateExport(String),
    DuplicateImport(String),
    DuplicateParams(String),
    ExistingFunction(String),
    ExistingImport(String),
    ExistingVariable(String),
    ImportedExport(String),
    MismatchingArrays(InfixOp),
    NestedArrays,
    RecursiveCall(String),
    ReservedKeyword(String),
    ReservedBuiltin(String),
    UndefinedFunction(String),
    UndefinedLocalVariable { function: String, variable: String },
    UndefinedVariable(String),
    WrongNumberOfArgs(String),
}

impl error::Error for YolkError {}

impl fmt::Display for YolkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            YolkError::ConflictingVariable(ident) => write!(f, "conflicting variable: {}", ident),
            YolkError::DuplicateExport(ident) => write!(f, "duplicate export: {}", ident),
            YolkError::DuplicateImport(ident) => write!(f, "duplicate import: {}", ident),
            YolkError::DuplicateParams(ident) => {
                write!(f, "function: {} has duplicate parameters", ident)
            }
            YolkError::ExistingFunction(ident) => {
                write!(f, "cannot redefine existing function: {}", ident)
            }
            YolkError::ExistingImport(ident) => {
                write!(f, "cannot import existing variable: {}", ident)
            }
            YolkError::ExistingVariable(ident) => {
                write!(f, "cannot reassign existing variable: {}", ident)
            }
            YolkError::ImportedExport(ident) => {
                write!(f, "cannot export imported variable: {}", ident)
            }
            YolkError::MismatchingArrays(op) => write!(
                f,
                "cannot apply operation: {:?} to arrays of different lengths",
                op
            ),
            YolkError::NestedArrays => write!(f, "cannot nest arrays"),
            YolkError::RecursiveCall(ident) => {
                write!(f, "cannot recursively call function: {}", ident)
            }
            YolkError::ReservedKeyword(keyword) => {
                write!(f, "cannot assign to reserved keyword: {}", keyword)
            }
            YolkError::ReservedBuiltin(builtin) => {
                write!(f, "cannot define reserved builtin: {}", builtin)
            }
            YolkError::UndefinedFunction(ident) => write!(f, "undefined function: {}", ident),
            YolkError::UndefinedLocalVariable { function, variable } => write!(
                f,
                "variable: {} is undefined in scope of function: {}",
                variable, function
            ),
            YolkError::UndefinedVariable(ident) => write!(f, "undefined variable: {}", ident),
            YolkError::WrongNumberOfArgs(ident) => {
                write!(f, "wrong number of arguments for function: {}", ident)
            }
        }
    }
}
