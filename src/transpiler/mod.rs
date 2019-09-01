use num_traits::identities::{One, Zero};
use yolol_number::YololNumber;

use crate::ast::{InfixOp, YolkExpr, YolkStmt, YololStmt};
use crate::environment::Environment;
use crate::error::TranspileError;
use crate::function::Function;
use crate::value::{ArrayExpr, NumberExpr, Value};

#[cfg(test)]
mod tests;

/// Transpiles Yolk statements to Yolol statements.
///
/// Returns Yolol statements.
///
/// # Panics
///
/// Panics if any of the nodes are not statements, or if any of the nodes are malformed.
pub fn transpile(stmts: &[YolkStmt]) -> Result<Vec<YololStmt>, TranspileError> {
    let mut env = Environment::new();
    let mut assigns = Vec::new();
    for stmt in stmts.iter() {
        match stmt {
            YolkStmt::Import { ident } => env.import(&ident)?,
            YolkStmt::Define {
                ident,
                params,
                body,
            } => env.define(&ident, Function::new(&ident, params, &*body)?)?,
            YolkStmt::Let { ident, expr } => {
                assigns.extend(env.let_value(&ident, expr_to_value(&env, &*expr)?)?);
            }
        }
    }
    Ok(assigns)
}

fn expr_to_value(env: &Environment, expr: &YolkExpr) -> Result<Value, TranspileError> {
    match expr {
        YolkExpr::Prefix { op, expr } => {
            let value = expr_to_value(env, &expr)?;
            Ok(value.apply_prefix_op(&op))
        }
        YolkExpr::Builtin { ident, args } => match ident.as_ref() {
            "sum" => sum_to_value(env, args),
            "product" => product_to_value(env, args),
            _ => panic!("expected builtin, but got: {:?}", ident),
        },
        YolkExpr::Call { ident, args } => {
            let function = env.function(ident)?;
            let expr = function.call(args)?;
            expr_to_value(env, &expr)
        }
        YolkExpr::Infix { lhs, op, rhs } => {
            let lhs = expr_to_value(env, &lhs)?;
            let rhs = expr_to_value(env, &rhs)?;
            lhs.apply_infix_op(&op, &rhs)
        }
        YolkExpr::Ident(s) => env.variable(s),
        YolkExpr::Literal(y) => Ok(Value::Number(NumberExpr::from_yolol_number(y.clone()))),
        YolkExpr::Array(exprs) => {
            let mut numbers = Vec::new();
            for expr in exprs.iter() {
                let value = expr_to_value(env, &expr)?;
                match value {
                    Value::Number(n) => numbers.push(n),
                    Value::Array(_) => return Err(TranspileError::NestedArrays),
                }
            }
            Ok(Value::Array(ArrayExpr::from_number_exprs(&numbers)))
        }
    }
}

fn sum_to_value(env: &Environment, args: &[YolkExpr]) -> Result<Value, TranspileError> {
    let mut values = Vec::new();
    for arg in args.iter() {
        values.push(expr_to_value(env, arg)?);
    }
    Ok(Value::reduce(
        &values,
        &InfixOp::Add,
        &NumberExpr::from_yolol_number(YololNumber::zero()),
    ))
}

fn product_to_value(env: &Environment, args: &[YolkExpr]) -> Result<Value, TranspileError> {
    let mut values = Vec::new();
    for arg in args.iter() {
        values.push(expr_to_value(env, arg)?);
    }
    Ok(Value::reduce(
        &values,
        &InfixOp::Mul,
        &NumberExpr::from_yolol_number(YololNumber::one()),
    ))
}
