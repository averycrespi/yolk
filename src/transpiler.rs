use crate::ast::{YolkNode, YololNode};
use crate::environment::Environment;
use crate::error::YolkError;
use crate::function::Function;
use crate::value::Value;

/// Transpiles Yolk statements to Yolol assign statements.
///
/// # Panics
///
/// Panics if any statements are malformed.
pub fn transpile(stmts: Vec<YolkNode>) -> Result<Vec<YololNode>, YolkError> {
    let mut env = Environment::new();
    let mut assigns = Vec::new();
    for stmt in stmts {
        match stmt {
            YolkNode::ImportStmt { ident } => env.import(&ident)?,
            YolkNode::DefineStmt {
                ident,
                params,
                body,
            } => {
                let function = Function::new(params, &*body)?;
                env.define(&ident, &function)?;
            }
            YolkNode::LetStmt { ident, expr } => {
                let value = expr_to_value(&env, &*expr)?;
                assigns.extend(env.let_value(&ident, &value)?);
            }
            YolkNode::ExportStmt { ident } => env.export(&ident)?,
            _ => panic!("unexpected statement: {:?}", stmt),
        }
    }
    Ok(assigns)
}

fn expr_to_value(env: &Environment, expr: &YolkNode) -> Result<Value, YolkError> {
    //TODO: implement
    Err(YolkError::NotImplemented)
}
