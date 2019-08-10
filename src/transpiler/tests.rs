use std::str::FromStr;

use yolol_number::YololNumber;

use crate::ast::{YolkNode, YololNode};
use crate::error::TranspileError;
use crate::transpiler::transpile;

#[test]
fn test_transpile_let_number() -> Result<(), TranspileError> {
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
fn test_transpile_let_array() -> Result<(), TranspileError> {
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
