use crate::ast::{InfixOp, YolkNode, YololNode};
use crate::environment::Environment;
use crate::error::YolkError;
use crate::function::Function;
use crate::value::{Array, Number, Value};

use std::collections::HashSet;

/// Transpiles Yolk statements to Yolol assign statements.
///
/// Returns assign statements and saved identifiers.
///
/// # Panics
///
/// Panics if any Yolk statements are malformed.
pub fn transpile(stmts: &[YolkNode]) -> Result<(Vec<YololNode>, HashSet<String>), YolkError> {
    let mut env = Environment::new();
    let mut assigns = Vec::new();
    for stmt in stmts.iter() {
        match stmt {
            YolkNode::ImportStmt { ident } => env.import(&ident)?,
            YolkNode::DefineStmt {
                ident,
                params,
                body,
            } => env.define(&ident, &Function::new(&ident, params, &*body)?)?,
            YolkNode::LetStmt { ident, expr } => {
                assigns.extend(env.let_value(&ident, &expr_to_value(&env, &*expr)?)?);
            }
            YolkNode::ExportStmt { ident } => env.export(&ident)?,
            _ => panic!("expected statement, but got: {:?}", stmt),
        }
    }
    Ok((assigns, env.saved()))
}

fn expr_to_value(env: &Environment, expr: &YolkNode) -> Result<Value, YolkError> {
    match expr {
        YolkNode::PrefixExpr { op, expr } => {
            let value = expr_to_value(env, &expr)?;
            Ok(value.apply_prefix_op(&op))
        }
        YolkNode::CallExpr { ident, args } => match ident.as_ref() {
            "sum" => sum_to_value(env, args),
            "product" => product_to_value(env, args),
            _ => {
                let function = env.function(ident)?;
                let expr = function.call(args)?;
                expr_to_value(env, &expr)
            }
        },
        YolkNode::InfixExpr { lhs, op, rhs } => {
            let lhs = expr_to_value(env, &lhs)?;
            let rhs = expr_to_value(env, &rhs)?;
            lhs.apply_infix_op(&op, &rhs)
        }
        YolkNode::Ident(s) => env.variable(s),
        YolkNode::Literal(f) => Ok(Value::Number(Number::from_float(*f))),
        YolkNode::Array(exprs) => {
            let mut numbers = Vec::new();
            for expr in exprs.iter() {
                let value = expr_to_value(env, &expr)?;
                match value {
                    Value::Number(n) => numbers.push(n),
                    Value::Array(_) => return Err(YolkError::NestedArrays),
                }
            }
            Ok(Value::Array(Array::from_numbers(&numbers)))
        }
        _ => panic!("expected YOlk expression, but got: {:?}", expr),
    }
}

fn sum_to_value(env: &Environment, args: &[YolkNode]) -> Result<Value, YolkError> {
    let mut values = Vec::new();
    for arg in args.iter() {
        values.push(expr_to_value(env, arg)?);
    }
    Ok(Value::reduce(
        &values,
        &InfixOp::Add,
        &Number::from_float(0.0),
    ))
}

fn product_to_value(env: &Environment, args: &[YolkNode]) -> Result<Value, YolkError> {
    let mut values = Vec::new();
    for arg in args.iter() {
        values.push(expr_to_value(env, arg)?);
    }
    Ok(Value::reduce(
        &values,
        &InfixOp::Mul,
        &Number::from_float(1.0),
    ))
}
