use num_traits::identities::Zero;
use yolol_number::YololNumber;

use crate::ast::{InfixOp, PrefixOp, YololExpr, YololProgram, YololStmt};
use crate::error::YolkError;

#[test]
fn test_format_neg() -> Result<(), YolkError> {
    let yolol: YololProgram = vec![YololStmt::Assign {
        ident: "a".to_string(),
        expr: Box::new(YololExpr::Prefix {
            op: PrefixOp::Neg,
            expr: Box::new(YololExpr::Ident("b".to_string())),
        }),
    }]
    .into();
    assert_eq!(yolol.to_string(), "a=-b");
    Ok(())
}

#[test]
fn test_format_not() -> Result<(), YolkError> {
    let yolol: YololProgram = vec![YololStmt::Assign {
        ident: "a".to_string(),
        expr: Box::new(YololExpr::Prefix {
            op: PrefixOp::Not,
            expr: Box::new(YololExpr::Ident("b".to_string())),
        }),
    }]
    .into();
    assert_eq!(yolol.to_string(), "a=not b");
    Ok(())
}

#[test]
fn test_format_not_neg() -> Result<(), YolkError> {
    let yolol: YololProgram = vec![YololStmt::Assign {
        ident: "a".to_string(),
        expr: Box::new(YololExpr::Prefix {
            op: PrefixOp::Not,
            expr: Box::new(YololExpr::Prefix {
                op: PrefixOp::Neg,
                expr: Box::new(YololExpr::Ident("b".to_string())),
            }),
        }),
    }]
    .into();
    assert_eq!(yolol.to_string(), "a=not -b");
    Ok(())
}

#[test]
fn test_format_neg_not() -> Result<(), YolkError> {
    let yolol: YololProgram = vec![YololStmt::Assign {
        ident: "a".to_string(),
        expr: Box::new(YololExpr::Prefix {
            op: PrefixOp::Neg,
            expr: Box::new(YololExpr::Prefix {
                op: PrefixOp::Not,
                expr: Box::new(YololExpr::Ident("b".to_string())),
            }),
        }),
    }]
    .into();
    assert_eq!(yolol.to_string(), "a=-(not b)");
    Ok(())
}

#[test]
fn test_format_not_not() -> Result<(), YolkError> {
    let yolol: YololProgram = vec![YololStmt::Assign {
        ident: "a".to_string(),
        expr: Box::new(YololExpr::Prefix {
            op: PrefixOp::Not,
            expr: Box::new(YololExpr::Prefix {
                op: PrefixOp::Not,
                expr: Box::new(YololExpr::Ident("b".to_string())),
            }),
        }),
    }]
    .into();
    assert_eq!(yolol.to_string(), "a=not not b");
    Ok(())
}

#[test]
fn test_format_neg_neg() -> Result<(), YolkError> {
    let yolol: YololProgram = vec![YololStmt::Assign {
        ident: "a".to_string(),
        expr: Box::new(YololExpr::Prefix {
            op: PrefixOp::Neg,
            expr: Box::new(YololExpr::Prefix {
                op: PrefixOp::Neg,
                expr: Box::new(YololExpr::Ident("b".to_string())),
            }),
        }),
    }]
    .into();
    assert_eq!(yolol.to_string(), "a=--b");
    Ok(())
}

#[test]
fn test_format_add() -> Result<(), YolkError> {
    let yolol: YololProgram = vec![YololStmt::Assign {
        ident: "a".to_string(),
        expr: Box::new(YololExpr::Infix {
            lhs: Box::new(YololExpr::Ident("b".to_string())),
            op: InfixOp::Add,
            rhs: Box::new(YololExpr::Ident("c".to_string())),
        }),
    }]
    .into();
    assert_eq!(yolol.to_string(), "a=b+c");
    Ok(())
}

#[test]
fn test_format_div() -> Result<(), YolkError> {
    let yolol: YololProgram = vec![YololStmt::Assign {
        ident: "a".to_string(),
        expr: Box::new(YololExpr::Infix {
            lhs: Box::new(YololExpr::Ident("b".to_string())),
            op: InfixOp::Div,
            rhs: Box::new(YololExpr::Ident("c".to_string())),
        }),
    }]
    .into();
    assert_eq!(yolol.to_string(), "a=b/c");
    Ok(())
}

#[test]
fn test_format_add_div() -> Result<(), YolkError> {
    let yolol: YololProgram = vec![YololStmt::Assign {
        ident: "a".to_string(),
        expr: Box::new(YololExpr::Infix {
            lhs: Box::new(YololExpr::Ident("b".to_string())),
            op: InfixOp::Add,
            rhs: Box::new(YololExpr::Infix {
                lhs: Box::new(YololExpr::Ident("c".to_string())),
                op: InfixOp::Div,
                rhs: Box::new(YololExpr::Ident("d".to_string())),
            }),
        }),
    }]
    .into();
    assert_eq!(yolol.to_string(), "a=b+c/d");
    Ok(())
}

#[test]
fn test_format_div_add() -> Result<(), YolkError> {
    let yolol: YololProgram = vec![YololStmt::Assign {
        ident: "a".to_string(),
        expr: Box::new(YololExpr::Infix {
            lhs: Box::new(YololExpr::Ident("b".to_string())),
            op: InfixOp::Div,
            rhs: Box::new(YololExpr::Infix {
                lhs: Box::new(YololExpr::Ident("c".to_string())),
                op: InfixOp::Add,
                rhs: Box::new(YololExpr::Ident("d".to_string())),
            }),
        }),
    }]
    .into();
    assert_eq!(yolol.to_string(), "a=b/(c+d)");
    Ok(())
}

#[test]
fn test_format_add_add() -> Result<(), YolkError> {
    let yolol: YololProgram = vec![YololStmt::Assign {
        ident: "a".to_string(),
        expr: Box::new(YololExpr::Infix {
            lhs: Box::new(YololExpr::Ident("b".to_string())),
            op: InfixOp::Add,
            rhs: Box::new(YololExpr::Infix {
                lhs: Box::new(YololExpr::Ident("c".to_string())),
                op: InfixOp::Add,
                rhs: Box::new(YololExpr::Ident("d".to_string())),
            }),
        }),
    }]
    .into();
    assert_eq!(yolol.to_string(), "a=b+c+d");
    Ok(())
}

#[test]
fn test_format_div_div() -> Result<(), YolkError> {
    let yolol: YololProgram = vec![YololStmt::Assign {
        ident: "a".to_string(),
        expr: Box::new(YololExpr::Infix {
            lhs: Box::new(YololExpr::Ident("b".to_string())),
            op: InfixOp::Div,
            rhs: Box::new(YololExpr::Infix {
                lhs: Box::new(YololExpr::Ident("c".to_string())),
                op: InfixOp::Div,
                rhs: Box::new(YololExpr::Ident("d".to_string())),
            }),
        }),
    }]
    .into();
    assert_eq!(yolol.to_string(), "a=b/(c/d)");
    Ok(())
}

#[test]
fn test_format_literal() -> Result<(), YolkError> {
    let yolol: YololProgram = vec![YololStmt::Assign {
        ident: "a".to_string(),
        expr: Box::new(YololExpr::Literal(YololNumber::zero())),
    }]
    .into();
    assert_eq!(yolol.to_string(), "a=0");
    Ok(())
}

#[test]
fn test_format_ident() -> Result<(), YolkError> {
    let yolol: YololProgram = vec![YololStmt::Assign {
        ident: "a".to_string(),
        expr: Box::new(YololExpr::Ident("b".to_string())),
    }]
    .into();
    assert_eq!(yolol.to_string(), "a=b");
    Ok(())
}
