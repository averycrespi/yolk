use std::error;
use std::fmt;

/// Represents a general error.
#[derive(Debug, Clone)]
pub enum Error {
    ParseError(ParseError),
    TranspileError(TranspileError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ParseError(e) => write!(f, "{}", e),
            Error::TranspileError(e) => write!(f, "{}", e),
        }
    }
}

impl error::Error for Error {}

impl From<ParseError> for Error {
    fn from(error: ParseError) -> Self {
        Error::ParseError(error)
    }
}

impl From<TranspileError> for Error {
    fn from(error: TranspileError) -> Self {
        Error::TranspileError(error)
    }
}

/// Represents an error during parsing.
#[derive(Debug, Clone)]
pub enum ParseError {
    BadSyntax(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::BadSyntax(s) => write!(f, "syntax error: {}", s),
        }
    }
}

/// Represents an error during transpilation.
#[derive(Debug, Clone)]
pub enum TranspileError {
    // Import errors
    ImportExisting(String),
    ImportKeyword(String),
    ImportTwice(String),

    // Define errors
    DefineKeyword(String),
    RedefineFunction(String),

    // Assign errors
    AssignSameLowercase(String),
    AssignToKeyword(String),
    ReassignVariable(String),

    // Access errors
    GetUndefinedFunction(String),
    GetUndefinedLocal(String),
    GetUndefinedVariable(String),

    // Function errors
    DuplicateParams,
    RecursiveCall,
    WrongNumberOfArgs(String),

    // Type errors
    MismatchedArrays,
    NestedArrays,
}

impl fmt::Display for TranspileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TranspileError::ImportExisting(variable) => {
                write!(f, "cannot import existing variable: {}", variable)
            }
            TranspileError::ImportKeyword(keyword) => {
                write!(f, "cannot import reserved keyword: {}", keyword)
            }
            TranspileError::ImportTwice(variable) => write!(f, "duplicate import: {}", variable),
            TranspileError::DefineKeyword(keyword) => {
                write!(f, "cannot define reserved keyword: {}", keyword)
            }
            TranspileError::RedefineFunction(function) => {
                write!(f, "cannot redefine function: {}", function)
            }
            TranspileError::AssignSameLowercase(variable) => write!(
                f,
                "multiple variable must not have the same lowercase representation: {}",
                variable
            ),
            TranspileError::AssignToKeyword(keyword) => {
                write!(f, "cannot assign to keyword: {}", keyword)
            }
            TranspileError::ReassignVariable(variable) => {
                write!(f, "cannot reassign variable: {}", variable)
            }
            TranspileError::GetUndefinedFunction(function) => {
                write!(f, "undefined function: {}", function)
            }
            TranspileError::GetUndefinedLocal(local) => {
                write!(f, "undefined local variable: {}", local)
            }
            TranspileError::GetUndefinedVariable(variable) => {
                write!(f, "undefined variable: {}", variable)
            }
            TranspileError::DuplicateParams => write!(f, "duplicate function parameters"),
            TranspileError::RecursiveCall => write!(f, "cannot define function recursively"),
            TranspileError::WrongNumberOfArgs(function) => {
                write!(f, "wrong number of args for function: {}", function)
            }
            TranspileError::MismatchedArrays => write!(f, "cannot operate on mismatched arrays"),
            TranspileError::NestedArrays => write!(f, "cannot create nested arrays"),
        }
    }
}
