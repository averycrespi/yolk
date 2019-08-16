use yolol_number::YololNumber;

use crate::ast::YolkNode;
use crate::error::TranspileError;
use crate::function::Function;

use std::str::FromStr;

#[test]
fn test_new() -> Result<(), TranspileError> {
    let function = Function::new(
        "function",
        &vec!["a".to_string(), "b".to_string(), "c".to_string()],
        &YolkNode::Ident("a".to_string()),
    )?;
    function.call(&vec![
        YolkNode::Literal(YololNumber::from_str("0").unwrap()),
        YolkNode::Literal(YololNumber::from_str("1").unwrap()),
        YolkNode::Literal(YololNumber::from_str("2").unwrap()),
    ])?;
    Ok(())
}

#[test]
#[should_panic]
fn test_duplicate_params() {
    Function::new(
        "function",
        &vec!["a".to_string(), "a".to_string()],
        &YolkNode::Ident("a".to_string()),
    )
    .unwrap();
}

#[test]
#[should_panic]
fn test_recursive_call() {
    Function::new(
        "function",
        &vec!["a".to_string()],
        &YolkNode::CallExpr {
            ident: "function".to_string(),
            args: vec![YolkNode::Ident("a".to_string())],
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
        &YolkNode::Ident("b".to_string()),
    )
    .unwrap();
}

#[test]
#[should_panic]
fn test_wrong_number_of_args() {
    let function = Function::new(
        "function",
        &vec!["a".to_string()],
        &YolkNode::Ident("a".to_string()),
    )
    .unwrap();
    function
        .call(&vec![
            YolkNode::Literal(YololNumber::from_str("0").unwrap()),
            YolkNode::Literal(YololNumber::from_str("1").unwrap()),
        ])
        .unwrap();
}
