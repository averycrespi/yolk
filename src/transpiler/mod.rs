use crate::ast::{InfixOp, YolkNode, YololNode};
use crate::environment::{Context, Environment};
use crate::error::TranspileError;
use crate::function::Function;
use crate::number::YololNumber;
use crate::value::{ArrayExpr, NumberExpr, Value};

#[cfg(test)]
mod tests;

/// Transpiles Yolk statements to Yolol assign statements.
///
/// Returns assign statements and the program context.
pub fn transpile(stmts: &[YolkNode]) -> Result<(Vec<YololNode>, Context), TranspileError> {
    let mut env = Environment::new();
    let mut assigns = Vec::new();
    for stmt in stmts.iter() {
        match stmt {
            YolkNode::ImportStmt { ident } => env.import(&ident)?,
            YolkNode::DefineStmt {
                ident,
                params,
                body,
            } => env.define(&ident, Function::new(&ident, params, &*body)?)?,
            YolkNode::LetStmt { ident, expr } => {
                assigns.extend(env.let_value(&ident, expr_to_value(&env, &*expr)?)?);
            }
            YolkNode::ExportStmt { ident } => env.export(&ident)?,
            _ => panic!("expected statement, but got: {:?}", stmt),
        }
    }
    Ok((assigns, env.context()))
}

fn expr_to_value(env: &Environment, expr: &YolkNode) -> Result<Value, TranspileError> {
    match expr {
        YolkNode::PrefixExpr { op, expr } => {
            let value = expr_to_value(env, &expr)?;
            Ok(value.apply_prefix_op(&op))
        }
        YolkNode::BuiltinExpr { ident, args } => match ident.as_ref() {
            "sum" => sum_to_value(env, args),
            "product" => product_to_value(env, args),
            _ => panic!("expected builtin, but got: {:?}", ident),
        },
        YolkNode::CallExpr { ident, args } => {
            let function = env.function(ident)?;
            let expr = function.call(args)?;
            expr_to_value(env, &expr)
        }
        YolkNode::InfixExpr { lhs, op, rhs } => {
            let lhs = expr_to_value(env, &lhs)?;
            let rhs = expr_to_value(env, &rhs)?;
            lhs.apply_infix_op(&op, &rhs)
        }
        YolkNode::Ident(s) => env.variable(s),
        YolkNode::Literal(y) => Ok(Value::Number(NumberExpr::from_yolol_number(y.clone()))),
        YolkNode::Array(exprs) => {
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
        _ => panic!("expected Yolk expression, but got: {:?}", expr),
    }
}

fn sum_to_value(env: &Environment, args: &[YolkNode]) -> Result<Value, TranspileError> {
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

fn product_to_value(env: &Environment, args: &[YolkNode]) -> Result<Value, TranspileError> {
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
