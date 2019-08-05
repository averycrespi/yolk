use crate::ast::YolkNode;
use crate::error::YolkError;

#[derive(Debug, Clone)]
pub struct Function {}

impl Function {
    pub fn new(params: Vec<String>, body: &YolkNode) -> Result<Function, YolkError> {
        //TODO: implement
        Ok(Function {})
    }
}
