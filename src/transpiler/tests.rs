use yolol_number::YololNumber;

use crate::ast::{InfixOp, YolkExpr};
use crate::error::YolkError;
use crate::transpiler::environment::Environment;
use crate::transpiler::function::Function;
use crate::transpiler::value::{ArrayExpr, NumberExpr, Value};

use std::str::FromStr;

#[test]
fn test_import() -> Result<(), YolkError> {
    let mut env = Environment::new();
    env.import("number")?;
    Ok(())
}

#[test]
fn test_define() -> Result<(), YolkError> {
    let mut env = Environment::new();
    let function = Function::new(
        "function",
        &vec!["a".to_string()],
        &YolkExpr::Ident("a".to_string()),
    )?;
    env.define("function", function)?;
    Ok(())
}

#[test]
fn test_let_value() -> Result<(), YolkError> {
    let mut env = Environment::new();
    let value = Value::Number(NumberExpr::from_yolol_number(
        YololNumber::from_str("0").unwrap(),
    ));
    env.let_value("number", value)?;
    env.variable("number")?;
    Ok(())
}

#[test]
#[should_panic]
fn test_get_undefined_variable() {
    let env = Environment::new();
    env.variable("number").unwrap();
}

#[test]
#[should_panic]
fn test_get_undefined_function() {
    let env = Environment::new();
    env.function("function").unwrap();
}

#[test]
#[should_panic]
fn test_import_twice() {
    let mut env = Environment::new();
    env.import("number").unwrap();
    env.import("number").unwrap();
}

#[test]
#[should_panic]
fn test_import_existing() {
    let mut env = Environment::new();
    let value = Value::Number(NumberExpr::from_yolol_number(
        YololNumber::from_str("0").unwrap(),
    ));
    env.let_value("number", value).unwrap();
    env.import("number").unwrap();
}

#[test]
#[should_panic]
fn test_import_keyword() {
    let mut env = Environment::new();
    env.import("sum").unwrap();
}

#[test]
#[should_panic]
fn test_redefine_function() {
    let mut env = Environment::new();
    let function = Function::new(
        "function",
        &vec!["a".to_string()],
        &YolkExpr::Ident("a".to_string()),
    )
    .unwrap();
    env.define("function", function.clone()).unwrap();
    env.define("function", function.clone()).unwrap();
}

#[test]
#[should_panic]
fn test_define_keyword() {
    let mut env = Environment::new();
    let function = Function::new(
        "sum",
        &vec!["a".to_string()],
        &YolkExpr::Ident("a".to_string()),
    )
    .unwrap();
    env.define("sum", function).unwrap();
}

#[test]
#[should_panic]
fn test_reassign_variable() {
    let mut env = Environment::new();
    let value = Value::Number(NumberExpr::from_yolol_number(
        YololNumber::from_str("0").unwrap(),
    ));
    env.let_value("number", value.clone()).unwrap();
    env.let_value("number", value.clone()).unwrap();
}

#[test]
#[should_panic]
fn test_assign_to_keyword() {
    let mut env = Environment::new();
    let value = Value::Number(NumberExpr::from_yolol_number(
        YololNumber::from_str("0").unwrap(),
    ));
    env.let_value("sum", value.clone()).unwrap();
}

#[test]
#[should_panic]
fn test_assign_same_lowercase() {
    let mut env = Environment::new();
    let value = Value::Number(NumberExpr::from_yolol_number(
        YololNumber::from_str("0").unwrap(),
    ));
    env.let_value("number", value.clone()).unwrap();
    env.let_value("NUMBER", value.clone()).unwrap();
}

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

#[test]
fn test_new() -> Result<(), YolkError> {
    let function = Function::new(
        "function",
        &vec!["a".to_string(), "b".to_string(), "c".to_string()],
        &YolkExpr::Ident("a".to_string()),
    )?;
    function.call(&vec![
        YolkExpr::Literal(YololNumber::from_str("0").unwrap()),
        YolkExpr::Literal(YololNumber::from_str("1").unwrap()),
        YolkExpr::Literal(YololNumber::from_str("2").unwrap()),
    ])?;
    Ok(())
}

#[test]
#[should_panic]
fn test_duplicate_params() {
    Function::new(
        "function",
        &vec!["a".to_string(), "a".to_string()],
        &YolkExpr::Ident("a".to_string()),
    )
    .unwrap();
}

#[test]
#[should_panic]
fn test_recursive_call() {
    Function::new(
        "function",
        &vec!["a".to_string()],
        &YolkExpr::Call {
            ident: "function".to_string(),
            args: vec![YolkExpr::Ident("a".to_string())],
        },
    )
    .unwrap();
}

#[test]
#[should_panic]
fn test_undefined_local() {
    Function::new(
        "function",
        &vec!["a".to_string()],
        &YolkExpr::Ident("b".to_string()),
    )
    .unwrap();
}

#[test]
#[should_panic]
fn test_wrong_number_of_args() {
    let function = Function::new(
        "function",
        &vec!["a".to_string()],
        &YolkExpr::Ident("a".to_string()),
    )
    .unwrap();
    function
        .call(&vec![
            YolkExpr::Literal(YololNumber::from_str("0").unwrap()),
            YolkExpr::Literal(YololNumber::from_str("1").unwrap()),
        ])
        .unwrap();
}
