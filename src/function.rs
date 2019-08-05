use crate::ast::YolkNode;
use crate::error::YolkError;

/// Represents a Yolk function.
#[derive(Debug, Clone)]
pub struct Function {}

impl Function {
    /// Creates a new Yolk function.
    pub fn new(ident: &str, params: Vec<String>, body: &YolkNode) -> Result<Function, YolkError> {
        //TODO: implement
        Err(YolkError::NotImplemented)
    }

    /// Calls a function with arguments.
    pub fn call(&self, args: Vec<YolkNode>) -> Result<YolkNode, YolkError> {
        //TODO: implement
        Err(YolkError::NotImplemented)
    }
}
