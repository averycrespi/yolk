use crate::ast::{YolkNode, YololNode};
use crate::environment::Environment;
use crate::error::YolkError;
use crate::function::Function;
use crate::value::{Number, Value};

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
    match expr {
        YolkNode::PrefixExpr { op, expr } => match expr_to_value(env, expr)? {
            Value::Number(number) => Ok(Value::Number(number.apply_prefix_op(&op))),
            Value::Array(array) => Ok(Value::Array(array.apply_prefix_op(&op))),
        },
        //TODO: implement
        YolkNode::CallExpr { ident, args } => Err(YolkError::NotImplemented),
        //TODO: implement
        YolkNode::InfixExpr { lhs, op, rhs } => Err(YolkError::NotImplemented),
        YolkNode::Ident(s) => env.variable(s),
        YolkNode::Literal(f) => Ok(Value::Number(Number::from_yolk_node(expr))),
        //TODO: implement
        YolkNode::Array(nodes) => Err(YolkError::NotImplemented),
        _ => panic!("unexpected expression: {:?}", expr),
    }
}
