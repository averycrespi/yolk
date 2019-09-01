use crate::ast::InfixOp;
use crate::error::YolkError;
use crate::value::{ArrayExpr, NumberExpr, Value};

#[test]
fn test_apply_infix_op_number_number() -> Result<(), YolkError> {
    let number = Value::Number(NumberExpr::from_ident("a"));
    number.apply_infix_op(&InfixOp::Add, &number)?;
    Ok(())
}

#[test]
fn test_apply_infix_op_number_array() -> Result<(), YolkError> {
    let number = Value::Number(NumberExpr::from_ident("a"));
    let array = Value::Array(ArrayExpr::from_number_exprs(&vec![
        NumberExpr::from_ident("b"),
        NumberExpr::from_ident("c"),
    ]));
    number.apply_infix_op(&InfixOp::Add, &array)?;
    array.apply_infix_op(&InfixOp::Add, &number)?;
    Ok(())
}

#[test]
fn test_apply_infix_op_array_array() -> Result<(), YolkError> {
    let array = Value::Array(ArrayExpr::from_number_exprs(&vec![
        NumberExpr::from_ident("a"),
        NumberExpr::from_ident("c"),
    ]));
    array.apply_infix_op(&InfixOp::Add, &array)?;
    Ok(())
}

#[test]
#[should_panic]
fn test_mismatched_arrays() {
    let left = Value::Array(ArrayExpr::from_number_exprs(&vec![
        NumberExpr::from_ident("a"),
        NumberExpr::from_ident("b"),
    ]));
    let right = Value::Array(ArrayExpr::from_number_exprs(&vec![
        NumberExpr::from_ident("a"),
        NumberExpr::from_ident("b"),
        NumberExpr::from_ident("c"),
    ]));
    left.apply_infix_op(&InfixOp::Add, &right).unwrap();
}
