use num_traits::identities::Zero;
use yolol_number::YololNumber;

use crate::ast::{YolkExpr, YolkProgram, YolkStmt};
use crate::error::YolkError;

use std::str::FromStr;

#[test]
fn test_import() -> Result<(), YolkError> {
    let parsed: YolkProgram = "import number".parse()?;
    let expected: YolkProgram = vec![YolkStmt::Import {
        ident: "number".to_string(),
    }]
    .into();
    assert_eq!(parsed, expected);
    Ok(())
}

#[test]
fn test_let_number() -> Result<(), YolkError> {
    let cases = vec!["0", "1", "1.0", "-1", "-1.0", "1.234", "-1.234"];
    for case in cases.iter() {
        println!("case: {}", case);
        let parsed: YolkProgram = format!("let number = {}", case).parse()?;
        let expected: YolkProgram = vec![YolkStmt::Let {
            ident: "number".to_string(),
            expr: Box::new(YolkExpr::Literal(YololNumber::from_str(case).unwrap())),
        }]
        .into();
        assert_eq!(parsed, expected);
    }
    Ok(())
}

#[test]
fn test_let_array() -> Result<(), YolkError> {
    let parsed: YolkProgram = "let array = [0, number]".parse()?;
    let expected: YolkProgram = vec![YolkStmt::Let {
        ident: "array".to_string(),
        expr: Box::new(YolkExpr::Array(vec![
            YolkExpr::Literal(YololNumber::zero()),
            YolkExpr::Ident("number".to_string()),
        ])),
    }]
    .into();
    assert_eq!(parsed, expected);
    Ok(())
}

#[test]
fn test_let_prefix() -> Result<(), YolkError> {
    let _: YolkProgram =
        "let number = not (abs (sqrt (sin (cos (tan (asin (acos (atan (0)))))))))".parse()?;
    Ok(())
}

#[test]
fn test_let_infix() -> Result<(), YolkError> {
    let _: YolkProgram =
        "let number = 1 + 2 - 3 * 4 / 5 % 6 ^ 7 < 8 <= 9 > 10 >= 11 == 12 != 13 and 14 or 15"
            .parse()?;
    Ok(())
}

#[test]
fn test_let_builtin() -> Result<(), YolkError> {
    let _: YolkProgram = "let number = sum([0, 1], 2) + product([0, 1], 2)".parse()?;
    Ok(())
}

#[test]
fn test_let_call() -> Result<(), YolkError> {
    let _: YolkProgram = "let number = func(0) + func([0, 1]) + func(number)".parse()?;
    Ok(())
}

#[test]
fn test_define() -> Result<(), YolkError> {
    let parsed: YolkProgram = "define identity(A) = A".parse()?;
    let expected: YolkProgram = vec![YolkStmt::Define {
        ident: "identity".to_string(),
        params: vec!["A".to_string()],
        body: Box::new(YolkExpr::Ident("A".to_string())),
    }]
    .into();
    assert_eq!(parsed, expected);
    Ok(())
}

#[test]
fn test_comment() -> Result<(), YolkError> {
    let _: YolkProgram = "// This is a comment".parse()?;
    Ok(())
}

#[test]
fn test_inline_comment() -> Result<(), YolkError> {
    let _: YolkProgram = "import number // This is a comment".parse()?;
    Ok(())
}

#[test]
fn test_extra_newlines() -> Result<(), YolkError> {
    let normal: YolkProgram = "let number = (0)".parse()?;
    let extra: YolkProgram = "let number = (\n0\n)\n".parse()?;
    assert_eq!(normal, extra);
    Ok(())
}

#[test]
#[should_panic]
fn test_invalid_ident() {
    let _: YolkProgram = "let !@#$%^&*() = 0".parse().unwrap();
}

#[test]
#[should_panic]
fn test_too_much_precision() {
    let _: YolkProgram = "let number = 1.2345".parse().unwrap();
}

#[test]
#[should_panic]
fn test_missing_whole() {
    let _: YolkProgram = "let number = .2345".parse().unwrap();
}

#[test]
#[should_panic]
fn test_missing_fraction() {
    let _: YolkProgram = "let number = 1.".parse().unwrap();
}

#[test]
#[should_panic]
fn test_missing_comma() {
    let _: YolkProgram = "let array = [0 1]".parse().unwrap();
}

#[test]
#[should_panic]
fn test_leading_comma() {
    let _: YolkProgram = "let array = [, 0 1]".parse().unwrap();
}

#[test]
#[should_panic]
fn test_trailing_comma() {
    let _: YolkProgram = "let array = [0 1,]".parse().unwrap();
}

#[test]
#[should_panic]
fn test_missing_bracket() {
    let _: YolkProgram = "let array = [0 1".parse().unwrap();
}

#[test]
#[should_panic]
fn test_missing_paren() {
    let _: YolkProgram = "let number = func(0".parse().unwrap();
}

#[test]
#[should_panic]
fn test_missing_whitespace() {
    let _: YolkProgram = "letnumber=0".parse().unwrap();
}
