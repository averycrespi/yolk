use num_traits::identities::{One, Zero};
use yolol_number::YololNumber;

use crate::ast::{InfixOp, PrefixOp, YololNode};
use crate::format::format_as_program;

#[test]
fn test_assign_a() {
    let stmts = vec![YololNode::AssignStmt {
        ident: "a".to_string(),
        expr: Box::new(YololNode::Literal(YololNumber::zero())),
    }];
    let program = format_as_program(&stmts);
    assert_eq!(program, "a=0".to_string())
}

#[test]
fn test_assign_variable() {
    let stmts = vec![YololNode::AssignStmt {
        ident: "a".to_string(),
        expr: Box::new(YololNode::Ident("b".to_string())),
    }];
    let program = format_as_program(&stmts);
    assert_eq!(program, "a=b".to_string())
}

#[test]
fn test_assign_prefix() {
    let stmts = vec![YololNode::AssignStmt {
        ident: "a".to_string(),
        expr: Box::new(YololNode::PrefixExpr {
            op: PrefixOp::Not,
            expr: Box::new(YololNode::Literal(YololNumber::zero())),
        }),
    }];
    let program = format_as_program(&stmts);
    assert_eq!(program, "a=not 0".to_string())
}

#[test]
fn test_assign_infix() {
    let stmts = vec![YololNode::AssignStmt {
        ident: "a".to_string(),
        expr: Box::new(YololNode::InfixExpr {
            lhs: Box::new(YololNode::Literal(YololNumber::zero())),
            op: InfixOp::Add,
            rhs: Box::new(YololNode::Literal(YololNumber::one())),
        }),
    }];
    let program = format_as_program(&stmts);
    assert_eq!(program, "a=0+1".to_string())
}

#[test]
fn test_assign_mul_then_add() {
    let stmts = vec![YololNode::AssignStmt {
        ident: "a".to_string(),
        expr: Box::new(YololNode::InfixExpr {
            lhs: Box::new(YololNode::Literal(YololNumber::zero())),
            op: InfixOp::Add,
            rhs: Box::new(YololNode::InfixExpr {
                lhs: Box::new(YololNode::Literal(YololNumber::zero())),
                op: InfixOp::Mul,
                rhs: Box::new(YololNode::Literal(YololNumber::one())),
            }),
        }),
    }];
    let program = format_as_program(&stmts);
    assert_eq!(program, "a=0+0*1".to_string())
}

#[test]
fn test_assign_add_then_mul() {
    let stmts = vec![YololNode::AssignStmt {
        ident: "a".to_string(),
        expr: Box::new(YololNode::InfixExpr {
            lhs: Box::new(YololNode::Literal(YololNumber::zero())),
            op: InfixOp::Mul,
            rhs: Box::new(YololNode::InfixExpr {
                lhs: Box::new(YololNode::Literal(YololNumber::zero())),
                op: InfixOp::Add,
                rhs: Box::new(YololNode::Literal(YololNumber::one())),
            }),
        }),
    }];
    let program = format_as_program(&stmts);
    assert_eq!(program, "a=0*(0+1)".to_string())
}

#[test]
fn test_assign_sub_then_sub() {
    let stmts = vec![YololNode::AssignStmt {
        ident: "a".to_string(),
        expr: Box::new(YololNode::InfixExpr {
            lhs: Box::new(YololNode::Ident("b".to_string())),
            op: InfixOp::Sub,
            rhs: Box::new(YololNode::InfixExpr {
                lhs: Box::new(YololNode::Ident("c".to_string())),
                op: InfixOp::Sub,
                rhs: Box::new(YololNode::Ident("d".to_string())),
            }),
        }),
    }];
    let program = format_as_program(&stmts);
    assert_eq!(program, "a=b-(c-d)".to_string())
}
