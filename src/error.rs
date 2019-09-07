/// Represents a general error.
#[derive(Debug, Fail)]
pub enum YolkError {
    #[fail(display = "invalid syntax: {}", msg)]
    InvalidSyntax { msg: String },

    #[fail(display = "cannot import existing variable: {}", var)]
    ImportExisting { var: String },
    #[fail(display = "cannot import keyword: {}", var)]
    ImportKeyword { var: String },

    #[fail(display = "cannot define existing function: {}", func)]
    DefineExisting { func: String },
    #[fail(display = "cannot define keyword: {}", func)]
    DefineKeyword { func: String },

    #[fail(display = "cannot assign to existing variable: {}", var)]
    AssignExisting { var: String },
    #[fail(display = "cannot assign to keyword: {}", var)]
    AssignKeyword { var: String },
    //TODO: improve message
    #[fail(display = "name conflict with variable: {}", var)]
    AssignConflict { var: String },

    #[fail(display = "undefined function: {}", func)]
    UndefinedFunction { func: String },
    #[fail(display = "undefined variable {}", var)]
    UndefinedVariable { var: String },

    #[fail(display = "duplicate parameters in function: {}", func)]
    DuplicateParams { func: String },
    #[fail(display = "recursive call in function: {}", func)]
    RecursiveCall { func: String },
    #[fail(display = "wrong number of arguments provided for function: {}", func)]
    WrongNumberOfArgs { func: String },

    //TODO: improve message
    #[fail(display = "mismatched array lengths")]
    MismatchedArrays,
    //TODO: improve message
    #[fail(display = "cannot nest arrays")]
    NestedArrays,
}
