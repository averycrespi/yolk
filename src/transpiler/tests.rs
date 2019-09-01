use std::str::FromStr;

use yolol_number::YololNumber;

use crate::ast::{YolkExpr, YolkStmt, YololExpr, YololStmt};
use crate::error::TranspileError;
use crate::transpiler::transpile;

#[test]
fn test_let_number() -> Result<(), TranspileError> {
    let yolk = vec![YolkStmt::Let {
        ident: "number".to_string(),
        expr: Box::new(YolkExpr::Literal(YololNumber::from_str("0").unwrap())),
    }];
    let yolol = transpile(&yolk)?;
    assert_eq!(
        yolol,
        vec![YololStmt::Assign {
            ident: "number".to_string(),
            expr: Box::new(YololExpr::Literal(YololNumber::from_str("0").unwrap()))
        }]
    );
    Ok(())
}

#[test]
fn test_let_array() -> Result<(), TranspileError> {
    let yolk = vec![YolkStmt::Let {
        ident: "array".to_string(),
        expr: Box::new(YolkExpr::Array(vec![
            YolkExpr::Literal(YololNumber::from_str("0").unwrap()),
            YolkExpr::Literal(YololNumber::from_str("1").unwrap()),
        ])),
    }];
    let yolol = transpile(&yolk)?;
    assert_eq!(
        yolol,
        vec![
            YololStmt::Assign {
                ident: "array_0".to_string(),
                expr: Box::new(YololExpr::Literal(YololNumber::from_str("0").unwrap()))
            },
            YololStmt::Assign {
                ident: "array_1".to_string(),
                expr: Box::new(YololExpr::Literal(YololNumber::from_str("1").unwrap()))
            }
        ]
    );
    Ok(())
}
