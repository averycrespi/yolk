use num_traits::identities::{One, Zero};
use yolol_number::YololNumber;

use crate::ast::{InfixOp, YolkNode, YololNode};
use crate::environment::{Context, Environment};
use crate::error::YolkError;
use crate::function::Function;
use crate::value::{ArrayExpr, NumberExpr, Value};

/// Transpiles Yolk statements to Yolol assign statements.
///
/// Returns assign statements and the program context.
pub fn transpile(stmts: &[YolkNode]) -> Result<(Vec<YololNode>, Context), YolkError> {
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
        YolkNode::Literal(y) => Ok(Value::Number(NumberExpr::from_yolol_number(y.clone()))),
        YolkNode::Array(exprs) => {
            let mut numbers = Vec::new();
            for expr in exprs.iter() {
                let value = expr_to_value(env, &expr)?;
                match value {
                    Value::Number(n) => numbers.push(n),
                    Value::Array(_) => return Err(YolkError::NestedArrays),
                }
            }
            Ok(Value::Array(ArrayExpr::from_number_exprs(&numbers)))
        }
        _ => panic!("expected Yolk expression, but got: {:?}", expr),
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
        &NumberExpr::from_yolol_number(YololNumber::zero()),
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
        &NumberExpr::from_yolol_number(YololNumber::one()),
    ))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use yolol_number::YololNumber;

    use crate::ast::{YolkNode, YololNode};
    use crate::error::YolkError;
    use crate::transpiler::transpile;

    #[test]
    fn test_transpile_let_number() -> Result<(), YolkError> {
        let yolk = vec![YolkNode::LetStmt {
            ident: "number".to_string(),
            expr: Box::new(YolkNode::Literal(YololNumber::from_str("0").unwrap())),
        }];
        let (yolol, _) = transpile(&yolk)?;
        assert_eq!(
            yolol,
            vec![YololNode::AssignStmt {
                ident: "number".to_string(),
                expr: Box::new(YololNode::Literal(YololNumber::from_str("0").unwrap()))
            }]
        );
        Ok(())
    }

    #[test]
    fn test_transpile_let_array() -> Result<(), YolkError> {
        let yolk = vec![YolkNode::LetStmt {
            ident: "array".to_string(),
            expr: Box::new(YolkNode::Array(vec![
                YolkNode::Literal(YololNumber::from_str("0").unwrap()),
                YolkNode::Literal(YololNumber::from_str("1").unwrap()),
            ])),
        }];
        let (yolol, _) = transpile(&yolk)?;
        assert_eq!(
            yolol,
            vec![
                YololNode::AssignStmt {
                    ident: "array_0".to_string(),
                    expr: Box::new(YololNode::Literal(YololNumber::from_str("0").unwrap()))
                },
                YololNode::AssignStmt {
                    ident: "array_1".to_string(),
                    expr: Box::new(YololNode::Literal(YololNumber::from_str("1").unwrap()))
                }
            ]
        );
        Ok(())
    }
}
