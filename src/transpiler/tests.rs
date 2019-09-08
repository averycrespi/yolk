use num_traits::identities::Zero;
use yolol_number::YololNumber;

use crate::ast::{InfixOp, YolkExpr};
use crate::error::YolkError;
use crate::transpiler::environment::Environment;
use crate::transpiler::function::Function;
use crate::transpiler::value::{Value, Vector};

use std::str::FromStr;

#[test]
fn test_env_import() -> Result<(), YolkError> {
    let mut env = Environment::new();
    env.import("number")?;
    Ok(())
}

#[test]
fn test_env_define() -> Result<(), YolkError> {
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
fn test_env_let_value() -> Result<(), YolkError> {
    let mut env = Environment::new();
    let value = Value::Scalar(YololNumber::zero().into());
    env.let_value("number", value)?;
    env.variable("number")?;
    Ok(())
}

#[test]
#[should_panic]
fn test_env_get_undefined_variable() {
    let env = Environment::new();
    env.variable("number").unwrap();
}

#[test]
#[should_panic]
fn test_env_get_undefined_function() {
    let env = Environment::new();
    env.function("function").unwrap();
}

#[test]
#[should_panic]
fn test_env_import_twice() {
    let mut env = Environment::new();
    env.import("number").unwrap();
    env.import("number").unwrap();
}

#[test]
#[should_panic]
fn test_env_import_existing() {
    let mut env = Environment::new();
    let value = Value::Scalar(YololNumber::zero().into());
    env.let_value("number", value).unwrap();
    env.import("number").unwrap();
}

#[test]
#[should_panic]
fn test_env_import_keyword() {
    let mut env = Environment::new();
    env.import("sum").unwrap();
}

#[test]
#[should_panic]
fn test_env_redefine_function() {
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
fn test_env_define_keyword() {
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
fn test_env_reassign_variable() {
    let mut env = Environment::new();
    let value = Value::Scalar(YololNumber::zero().into());
    env.let_value("number", value.clone()).unwrap();
    env.let_value("number", value.clone()).unwrap();
}

#[test]
#[should_panic]
fn test_env_assign_to_keyword() {
    let mut env = Environment::new();
    let value = Value::Scalar(YololNumber::zero().into());
    env.let_value("sum", value.clone()).unwrap();
}

#[test]
#[should_panic]
fn test_env_assign_same_lowercase() {
    let mut env = Environment::new();
    let value = Value::Scalar(YololNumber::zero().into());
    env.let_value("number", value.clone()).unwrap();
    env.let_value("NUMBER", value.clone()).unwrap();
}

#[test]
fn test_apply_infix_op_scalar_scalar() -> Result<(), YolkError> {
    let value = Value::Scalar("a".parse()?);
    value.apply_infix_op(&InfixOp::Add, &value)?;
    Ok(())
}

#[test]
fn test_apply_infix_op_scalar_vector() -> Result<(), YolkError> {
    let lhs = Value::Scalar("a".parse()?);
    let rhs = Value::Vector(Vector::from_expanded_ident("b", 3));
    lhs.apply_infix_op(&InfixOp::Add, &rhs)?;
    Ok(())
}

#[test]
fn test_apply_infix_op_vector_scalar() -> Result<(), YolkError> {
    let lhs = Value::Vector(Vector::from_expanded_ident("b", 3));
    let rhs = Value::Scalar("a".parse()?);
    lhs.apply_infix_op(&InfixOp::Add, &rhs)?;
    Ok(())
}

#[test]
fn test_apply_infix_op_vector_vector() -> Result<(), YolkError> {
    let lhs = Value::Vector(Vector::from_expanded_ident("a", 3));
    let rhs = Value::Vector(Vector::from_expanded_ident("b", 3));
    lhs.apply_infix_op(&InfixOp::Add, &rhs)?;
    Ok(())
}

#[test]
#[should_panic]
fn test_apply_infix_op_mismatched_arrays() {
    let lhs = Value::Vector(Vector::from_expanded_ident("a", 2));
    let rhs = Value::Vector(Vector::from_expanded_ident("b", 3));
    lhs.apply_infix_op(&InfixOp::Add, &rhs).unwrap();
}

#[test]
fn test_func_new() -> Result<(), YolkError> {
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
fn test_func_duplicate_params() {
    Function::new(
        "function",
        &vec!["a".to_string(), "a".to_string()],
        &YolkExpr::Ident("a".to_string()),
    )
    .unwrap();
}

#[test]
#[should_panic]
fn test_func_recursive_call() {
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
fn test_func_undefined_local() {
    Function::new(
        "function",
        &vec!["a".to_string()],
        &YolkExpr::Ident("b".to_string()),
    )
    .unwrap();
}

#[test]
#[should_panic]
fn test_func_wrong_number_of_args() {
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
