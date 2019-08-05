use crate::ast::YolkNode;
use crate::error::YolkError;

/// Represents a Yolk function.
#[derive(Debug, Clone)]
pub struct Function {}

impl Function {
    pub fn new(params: Vec<String>, body: &YolkNode) -> Result<Function, YolkError> {
        //TODO: implement
        Err(YolkError::NotImplemented)
    }
}
