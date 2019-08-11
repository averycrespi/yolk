use std::str::FromStr;


use crate::number::YololNumber;
use crate::ast::YolkNode;
use crate::error::ParseError;
use crate::parser::parse;

#[test]
fn test_parse_import() -> Result<(), ParseError> {
    assert_eq!(
        parse("import number;")?,
        vec![YolkNode::ImportStmt {
            ident: "number".to_string()
        }]
    );
    Ok(())
}

#[test]
fn test_parse_let_number() -> Result<(), ParseError> {
    let cases = vec!["0", "1", "1.0", "-1", "-1.0", "1.2345", "-1.2345"];
    for case in cases.iter() {
        println!("case: {}", case);
        assert_eq!(
            parse(&format!("let number = {};", case))?,
            vec![YolkNode::LetStmt {
                ident: "number".to_string(),
                expr: Box::new(YolkNode::Literal(YololNumber::from_str(case).unwrap()))
            }]
        );
    }
    Ok(())
}

#[test]
fn test_parse_let_array() -> Result<(), ParseError> {
    assert_eq!(
        parse("let array = [0, number];")?,
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
fn test_parse_let_prefix() -> Result<(), ParseError> {
    //TODO: check results
    parse("let number = not (abs (sqrt (sin (cos (tan (asin (acos (atan (0)))))))));")?;
    Ok(())
}

#[test]
fn test_parse_let_infix() -> Result<(), ParseError> {
    //TODO: check results
    parse("let number = 1 + 2 - 3 * 4 / 5 % 6 ^ 7 < 8 <= 9 > 10 >= 11 == 12 != 13 and 14 or 15;")?;
    Ok(())
}

#[test]
fn test_parse_let_builtin() -> Result<(), ParseError> {
    //TODO: check results
    parse("let number = sum(0);")?;
    Ok(())
}

#[test]
fn test_parse_let_call() -> Result<(), ParseError> {
    //TODO: check results
    parse("let number = function(0) + function([0, 1]) + function(number);")?;
    Ok(())
}

#[test]
fn test_parse_define() -> Result<(), ParseError> {
    assert_eq!(
        parse("define identity(A) = A;")?,
        vec![YolkNode::DefineStmt {
            ident: "identity".to_string(),
            params: vec!["A".to_string()],
            body: Box::new(YolkNode::Ident("A".to_string()))
        }]
    );
    Ok(())
}

#[test]
fn test_parse_extra_whitespace() -> Result<(), ParseError> {
    assert_eq!(
        parse("let number = 0;")?,
        parse(" \n\tlet \n\tnumber \n\t= \n\t0 \n\t; \n\t")?
    );
    Ok(())
}

#[test]
#[should_panic]
fn test_parse_missing_semicolon() {
    parse("let number = 0").unwrap();
}

#[test]
#[should_panic]
fn test_parse_extra_semicolon() {
    parse("let number = 0;;").unwrap();
}

#[test]
#[should_panic]
#[allow(unused_variables)]
fn test_parse_invalid_ident() {
    parse("let !@#$%^&*() = 0;").unwrap();
}

#[test]
#[should_panic]
fn test_parse_too_much_precision() {
    parse("let number = 1.23456;").unwrap();
}

#[test]
#[should_panic]
fn test_parse_missing_whole() {
    parse("let number = .0;").unwrap();
}

#[test]
#[should_panic]
fn test_parse_missing_fraction() {
    parse("let number = 1.;").unwrap();
}
