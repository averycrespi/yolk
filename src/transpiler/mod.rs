use num_traits::identities::{One, Zero};
use yolol_number::YololNumber;

use crate::ast::{InfixOp, YolkExpr, YolkProgram, YolkStmt, YololProgram};
use crate::error::YolkError;

#[cfg(test)]
mod tests;

mod environment;
mod function;
mod value;

use environment::Environment;
use function::Function;
use value::Value;

/// Transpiles a Yolk program to a Yolol program
///
/// # Panics
///
/// Panics if the Yolk program is malformed.
pub fn transpile(program: YolkProgram) -> Result<YololProgram, YolkError> {
    let mut env = Environment::new();
    let mut assigns = Vec::new();
    for stmt in program.into_iter() {
        match stmt {
            YolkStmt::Import { ident } => env.import(&ident)?,
            YolkStmt::Define {
                ident,
                params,
                body,
            } => env.define(&ident, Function::new(&ident, &params, &*body)?)?,
            YolkStmt::Let { ident, expr } => {
                assigns.extend(env.let_value(&ident, expr_to_value(&env, &*expr)?)?);
            }
        }
    }
    Ok(assigns.into())
}

fn expr_to_value(env: &Environment, expr: &YolkExpr) -> Result<Value, YolkError> {
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
        YolkExpr::Literal(y) => Ok(Value::Scalar(y.clone().into())),
        YolkExpr::Array(exprs) => {
            let mut scalars = Vec::new();
            for expr in exprs.iter() {
                let value = expr_to_value(env, &expr)?;
                match value {
                    Value::Scalar(s) => scalars.push(s),
                    Value::Vector(_) => return Err(YolkError::NestedArrays),
                }
            }
            Ok(Value::Vector(scalars.into()))
        }
    }
}

fn sum_to_value(env: &Environment, args: &[YolkExpr]) -> Result<Value, YolkError> {
    let mut values = Vec::new();
    for arg in args.iter() {
        values.push(expr_to_value(env, arg)?);
    }
    Ok(Value::left_fold(
        &values,
        &InfixOp::Add,
        &YololNumber::zero().into(),
    ))
}

fn product_to_value(env: &Environment, args: &[YolkExpr]) -> Result<Value, YolkError> {
    let mut values = Vec::new();
    for arg in args.iter() {
        values.push(expr_to_value(env, arg)?);
    }
    Ok(Value::left_fold(
        &values,
        &InfixOp::Mul,
        &YololNumber::one().into(),
    ))
}
