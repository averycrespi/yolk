use crate::ast::{YolkNode, YololNode};
use crate::environment::Environment;
use crate::error::YolkError;
use crate::function::Function;
use crate::value::{Array, Number, Value};

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
                let function = Function::new(&ident, params, &*body)?;
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
        YolkNode::PrefixExpr { op, expr } => {
            let value = expr_to_value(env, &expr)?;
            Ok(value.apply_prefix_op(&op))
        }
        YolkNode::CallExpr { ident, args } => {
            let function = env.function(ident)?;
            let expr = function.call(args.to_vec())?;
            expr_to_value(env, &expr)
        }
        YolkNode::InfixExpr { lhs, op, rhs } => {
            let lhs = expr_to_value(env, &lhs)?;
            let rhs = expr_to_value(env, &rhs)?;
            lhs.apply_infix_op(&op, &rhs)
        }
        YolkNode::Ident(s) => env.variable(s),
        YolkNode::Literal(_) => Ok(Value::Number(Number::from_yolk_node(expr))),
        YolkNode::Array(exprs) => {
            let mut numbers = Vec::new();
            for expr in exprs.iter() {
                let value = expr_to_value(env, &expr)?;
                match value {
                    Value::Number(n) => numbers.push(n),
                    Value::Array(_) => return Err(YolkError::NestedArrays),
                }
            }
            Ok(Value::Array(Array::from_numbers(numbers)))
        }
        _ => panic!("unexpected expression: {:?}", expr),
    }
}
