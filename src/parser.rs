use std::str::FromStr;

use pest::Parser;
use yolol_number::YololNumber;

use crate::ast::{InfixOp, PrefixOp, YolkNode};
use crate::error::YolkError;

#[derive(Parser)]
#[grammar = "grammar/yolk.pest"]
pub struct YolkParser;

/// Parses Yolk statements from source text.
pub fn parse(source: &str) -> Result<Vec<YolkNode>, YolkError> {
    let mut ast = vec![];
    let pairs = YolkParser::parse(Rule::program, source)
        .map_err(|e| YolkError::BadSyntax(e.to_string()))?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::import_stmt => ast.push(parse_import_stmt(pair)),
            Rule::define_stmt => ast.push(parse_define_stmt(pair)),
            Rule::let_stmt => ast.push(parse_let_stmt(pair)),
            Rule::export_stmt => ast.push(parse_export_stmt(pair)),
            Rule::EOI => (),
            _ => panic!("expected rule statement, but got: {:?}", pair),
        }
    }
    Ok(ast)
}

fn parse_import_stmt(stmt: pest::iterators::Pair<Rule>) -> YolkNode {
    let mut pair = stmt.clone().into_inner();
    let ident = pair.next().expect("failed to unwrap ident from pair");
    YolkNode::ImportStmt {
        ident: ident.as_str().to_string(),
    }
}

fn parse_define_stmt(stmt: pest::iterators::Pair<Rule>) -> YolkNode {
    let mut pair = stmt.clone().into_inner();
    let ident = pair.next().expect("failed to unwrap ident from pair");
    let params = pair.next().expect("failed to unwrap params from pair");
    let body = pair.next().expect("failed to unwrap body from pair");
    YolkNode::DefineStmt {
        ident: ident.as_str().to_string(),
        params: params
            .into_inner()
            .map(|x| x.as_str().to_string())
            .collect(),
        body: Box::new(parse_expr(body)),
    }
}

fn parse_let_stmt(stmt: pest::iterators::Pair<Rule>) -> YolkNode {
    let mut pair = stmt.clone().into_inner();
    let ident = pair.next().expect("failed to unwrap ident from pair");
    let expr = pair.next().expect("failed to unwrap expr from pair");
    YolkNode::LetStmt {
        ident: ident.as_str().to_string(),
        expr: Box::new(parse_expr(expr)),
    }
}

fn parse_export_stmt(stmt: pest::iterators::Pair<Rule>) -> YolkNode {
    let mut pair = stmt.clone().into_inner();
    let ident = pair.next().expect("failed to unwrap ident from pair");
    YolkNode::ExportStmt {
        ident: ident.as_str().to_string(),
    }
}

fn parse_expr(expr: pest::iterators::Pair<Rule>) -> YolkNode {
    match expr.as_rule() {
        Rule::prefix_expr => {
            let mut pair = expr.into_inner();
            let op = pair.next().expect("failed to unwrap op from pair");
            let expr = pair.next().expect("failed to unwrap expr from pair");
            parse_prefix_expr(op, parse_expr(expr))
        }
        Rule::call_expr => {
            let mut pair = expr.into_inner();
            let ident = pair.next().expect("failed to unwrap ident from pair");
            let args = pair.next().expect("failed to unwrap args from pair");
            YolkNode::CallExpr {
                ident: ident.as_str().to_string(),
                args: args.into_inner().map(parse_expr).collect(),
            }
        }
        Rule::infix_expr => {
            let mut pair = expr.into_inner();
            let lhs = pair.next().expect("failed to unwrap lhs from pair");
            let op = pair.next().expect("failed to unwrap op from pair");
            let rhs = pair.next().expect("failed to unwrap rhs from pair");
            parse_infix_expr(parse_expr(lhs), op, parse_expr(rhs))
        }
        Rule::ident => YolkNode::Ident(expr.as_str().to_string()),
        Rule::literal => YolkNode::Literal(
            YololNumber::from_str(expr.as_str())
                .unwrap_or_else(|e| panic!("failed to parse YololNumber from string: {}", e)),
        ),
        Rule::array => {
            let exprs: Vec<YolkNode> = expr.into_inner().map(parse_expr).collect();
            YolkNode::Array(exprs)
        }
        _ => panic!("expected rule expression, but got: {:?}", expr),
    }
}

fn parse_prefix_expr(op: pest::iterators::Pair<Rule>, expr: YolkNode) -> YolkNode {
    YolkNode::PrefixExpr {
        op: match op.as_str() {
            "not" => PrefixOp::Not,
            "abs" => PrefixOp::Abs,
            "sqrt" => PrefixOp::Sqrt,
            "sin" => PrefixOp::Sin,
            "cos" => PrefixOp::Cos,
            "tan" => PrefixOp::Tan,
            "asin" => PrefixOp::Asin,
            "acos" => PrefixOp::Acos,
            "atan" => PrefixOp::Atan,
            _ => panic!("expected prefix op, but got: {:?}", op),
        },
        expr: Box::new(expr),
    }
}

fn parse_infix_expr(lhs: YolkNode, op: pest::iterators::Pair<Rule>, rhs: YolkNode) -> YolkNode {
    YolkNode::InfixExpr {
        lhs: Box::new(lhs),
        op: match op.as_str() {
            "+" => InfixOp::Add,
            "-" => InfixOp::Sub,
            "*" => InfixOp::Mul,
            "/" => InfixOp::Div,
            "%" => InfixOp::Mod,
            "^" => InfixOp::Exp,
            "<" => InfixOp::LessThan,
            "<=" => InfixOp::LessEqual,
            ">" => InfixOp::GreaterThan,
            ">=" => InfixOp::GreaterEqual,
            "==" => InfixOp::Equal,
            "!=" => InfixOp::NotEqual,
            "and" => InfixOp::And,
            "or" => InfixOp::Or,
            _ => panic!("expected infix op, but got: {:?}", op),
        },
        rhs: Box::new(rhs),
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use yolol_number::YololNumber;

    use crate::ast::YolkNode;
    use crate::error::YolkError;
    use crate::parser::parse;

    #[test]
    fn test_parse_import() -> Result<(), YolkError> {
        assert_eq!(
            parse("import variable;")?,
            vec![YolkNode::ImportStmt {
                ident: "variable".to_string()
            }]
        );
        Ok(())
    }

    #[test]
    fn test_parse_let_number() -> Result<(), YolkError> {
        assert_eq!(
            parse("let number = 0;")?,
            vec![YolkNode::LetStmt {
                ident: "number".to_string(),
                expr: Box::new(YolkNode::Literal(YololNumber::from_str("0").unwrap()))
            }]
        );
        Ok(())
    }

    #[test]
    fn test_parse_let_array() -> Result<(), YolkError> {
        assert_eq!(
            parse("let array = [0, 1];")?,
            vec![YolkNode::LetStmt {
                ident: "array".to_string(),
                expr: Box::new(YolkNode::Array(vec![
                    YolkNode::Literal(YololNumber::from_str("0").unwrap()),
                    YolkNode::Literal(YololNumber::from_str("1").unwrap())
                ]))
            }]
        );
        Ok(())
    }

    #[test]
    fn test_parse_define() -> Result<(), YolkError> {
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
    fn test_parse_extra_newlines() -> Result<(), YolkError> {
        assert_eq!(
            parse("let number = 0;")?,
            parse("\nlet\nnumber\n=\n0\n;\n")?
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
    #[allow(unused_variables)]
    fn test_parse_invalid_ident() {
        parse("let !@#$%^&*() = 0;").unwrap();
    }
}
