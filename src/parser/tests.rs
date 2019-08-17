use std::str::FromStr;

use yolol_number::YololNumber;

use crate::ast::YolkNode;
use crate::error::ParseError;
use crate::parser::parse;

#[test]
fn test_import() -> Result<(), ParseError> {
    assert_eq!(
        parse("import number")?,
        vec![YolkNode::ImportStmt {
            ident: "number".to_string()
        }]
    );
    Ok(())
}

#[test]
fn test_let_number() -> Result<(), ParseError> {
    let cases = vec!["0", "1", "1.0", "-1", "-1.0", "1.2345", "-1.2345"];
    for case in cases.iter() {
        println!("case: {}", case);
        assert_eq!(
            parse(&format!("let number = {}", case))?,
            vec![YolkNode::LetStmt {
                ident: "number".to_string(),
                expr: Box::new(YolkNode::Literal(YololNumber::from_str(case).unwrap()))
            }]
        );
    }
    Ok(())
}

#[test]
fn test_let_array() -> Result<(), ParseError> {
    assert_eq!(
        parse("let array = [0, number]")?,
        vec![YolkNode::LetStmt {
            ident: "array".to_string(),
            expr: Box::new(YolkNode::Array(vec![
                YolkNode::Literal(YololNumber::from_str("0").unwrap()),
                YolkNode::Ident("number".to_string())
            ]))
        }]
    );
    Ok(())
}

#[test]
fn test_let_prefix() -> Result<(), ParseError> {
    parse("let number = not (abs (sqrt (sin (cos (tan (asin (acos (atan (0)))))))))")?;
    Ok(())
}

#[test]
fn test_let_infix() -> Result<(), ParseError> {
    parse("let number = 1 + 2 - 3 * 4 / 5 % 6 ^ 7 < 8 <= 9 > 10 >= 11 == 12 != 13 and 14 or 15")?;
    Ok(())
}

#[test]
fn test_let_builtin() -> Result<(), ParseError> {
    parse("let number = sum([0, 1], 2) + product([0, 1], 2)")?;
    Ok(())
}

#[test]
fn test_let_call() -> Result<(), ParseError> {
    parse("let number = function(0) + function([0, 1]) + function(number)")?;
    Ok(())
}

#[test]
fn test_define() -> Result<(), ParseError> {
    assert_eq!(
        parse("define identity(A) = A")?,
        vec![YolkNode::DefineStmt {
            ident: "identity".to_string(),
            params: vec!["A".to_string()],
            body: Box::new(YolkNode::Ident("A".to_string()))
        }]
    );
    Ok(())
}

#[test]
fn test_comment() -> Result<(), ParseError> {
    parse("// This is a comment")?;
    Ok(())
}

#[test]
fn test_inline_comment() -> Result<(), ParseError> {
    parse("import number // This is a comment")?;
    Ok(())
}

#[test]
fn test_extra_newlines() -> Result<(), ParseError> {
    assert_eq!(parse("let number = (0)")?, parse("let number = (\n0\n)")?,);
    Ok(())
}

#[test]
#[should_panic]
fn test_invalid_ident() {
    parse("let !@#$%^&*() = 0").unwrap();
}

#[test]
#[should_panic]
fn test_too_much_precision() {
    parse("let number = 1.23456").unwrap();
}

#[test]
#[should_panic]
fn test_missing_whole() {
    parse("let number = .0").unwrap();
}

#[test]
#[should_panic]
fn test_missing_fraction() {
    parse("let number = 1.").unwrap();
}

#[test]
#[should_panic]
fn test_missing_comma() {
    parse("let array = [0 1]").unwrap();
}

#[test]
#[should_panic]
fn test_leading_comma() {
    parse("let array = [, 0, 1]").unwrap();
}

#[test]
#[should_panic]
fn test_trailing_comma() {
    parse("let array = [0, 1, ]").unwrap();
}

#[test]
#[should_panic]
fn test_missing_bracket() {
    parse("let array = [0, 1, 2").unwrap();
}

#[test]
#[should_panic]
fn test_missing_paren() {
    parse("let number = function(0").unwrap();
}
