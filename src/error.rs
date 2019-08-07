use std::error;
use std::fmt;

/// Represents a general Yolk error.
#[derive(Debug, Clone)]
pub enum YolkError {
    // Parse errors
    BadSyntax(String),

    // Wrapped errors
    WithStmt { stmt: String, error: Box<YolkError> },

    // Import errors
    ImportExisting(String),
    ImportKeyword(String),
    ImportTwice(String),

    // Define errors
    DefineBuiltin(String),
    RedefineFunction(String),

    // Assign errors
    AssignInsensitive(String),
    AssignToKeyword(String),
    ReassignVariable(String),

    // Export errors
    ExportTwice(String),
    ExportUndefined(String),

    // Access errors
    GetUndefinedFunction(String),
    GetUndefinedLocal { function: String, local: String },
    GetUndefinedVariable(String),

    // Function errors
    DuplicateParams(String),
    RecursiveCall(String),
    WrongNumberOfArgs(String),

    // Value errors
    MismatchedArrays,
    NestedArrays,
}

impl error::Error for YolkError {}

impl fmt::Display for YolkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            YolkError::BadSyntax(message) => {
                write!(f, "syntax error at line:column {}", message.trim())
            }
            YolkError::WithStmt { stmt, error } => write!(f, "{}\nwith statement: {}", error, stmt),
            YolkError::ImportExisting(variable) => {
                write!(f, "cannot import existing variable: {}", variable)
            }
            YolkError::ImportKeyword(keyword) => {
                write!(f, "cannot import reserved keyword: {}", keyword)
            }
            YolkError::ImportTwice(variable) => {
                write!(f, "cannot import variable twice: {}", variable)
            }
            YolkError::DefineBuiltin(builtin) => {
                write!(f, "cannot define builtin function: {}", builtin)
            }
            YolkError::RedefineFunction(function) => {
                write!(f, "cannot redefine existing function: {}", function)
            }
            YolkError::AssignInsensitive(variable) => write!(
                f,
                "multiple variables cannot have the same case-insensitive name: {}",
                variable.to_lowercase()
            ),
            YolkError::AssignToKeyword(keyword) => {
                write!(f, "cannot assign to reserved keyword: {}", keyword)
            }
            YolkError::ReassignVariable(variable) => {
                write!(f, "cannot reassign existing variable: {}", variable)
            }
            YolkError::ExportTwice(variable) => {
                write!(f, "cannot export variable twice: {}", variable)
            }
            YolkError::ExportUndefined(variable) => {
                write!(f, "cannot export undefined variable: {}", variable)
            }
            YolkError::GetUndefinedFunction(function) => {
                write!(f, "undefined function: {}", function)
            }
            YolkError::GetUndefinedLocal { function, local } => {
                write!(f, "undefined local: {} in function: {}", local, function)
            }
            YolkError::GetUndefinedVariable(variable) => {
                write!(f, "undefined variable: {}", variable)
            }
            YolkError::DuplicateParams(function) => {
                write!(f, "duplicate parameters in function: {}", function)
            }
            YolkError::RecursiveCall(function) => {
                write!(f, "recursive call in function: {}", function)
            }
            YolkError::WrongNumberOfArgs(function) => {
                write!(f, "wrong number of args for function: {}", function)
            }
            YolkError::MismatchedArrays => write!(
                f,
                "cannot perform operation on arrays with different lengths"
            ),
            YolkError::NestedArrays => write!(f, "cannot create nested arrays"),
        }
    }
}
