use yolol_number::YololNumber;

use crate::ast::YolkExpr;
use crate::environment::Environment;
use crate::error::TranspileError;
use crate::function::Function;
use crate::value::{NumberExpr, Value};

use std::str::FromStr;

#[test]
fn test_import() -> Result<(), TranspileError> {
    let mut env = Environment::new();
    env.import("number")?;
    Ok(())
}

#[test]
fn test_define() -> Result<(), TranspileError> {
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
fn test_let_value() -> Result<(), TranspileError> {
    let mut env = Environment::new();
    let value = Value::Number(NumberExpr::from_yolol_number(
        YololNumber::from_str("0").unwrap(),
    ));
    env.let_value("number", value)?;
    env.variable("number")?;
    Ok(())
}

#[test]
fn test_export() -> Result<(), TranspileError> {
    let mut env = Environment::new();
    let value = Value::Number(NumberExpr::from_yolol_number(
        YololNumber::from_str("0").unwrap(),
    ));
    env.let_value("number", value)?;
    env.export("number")?;
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
#[should_panic]
fn test_export_twice() {
    let mut env = Environment::new();
    let value = Value::Number(NumberExpr::from_yolol_number(
        YololNumber::from_str("0").unwrap(),
    ));
    env.let_value("number", value.clone()).unwrap();
    env.export("number").unwrap();
    env.export("number").unwrap();
}
