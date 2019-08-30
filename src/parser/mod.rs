use std::str::FromStr;

use num_traits::identities::Zero;
use pest::iterators::Pair;
use pest::prec_climber::{Assoc, Operator, PrecClimber};
use pest::Parser;
use yolol_number::YololNumber;

use crate::ast::{InfixOp, PrefixOp, YolkNode};
use crate::error::ParseError;

#[cfg(test)]
mod tests;

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = build_prec_climber();
}

fn build_prec_climber() -> PrecClimber<Rule> {
    PrecClimber::new(vec![
        Operator::new(Rule::logical_or, Assoc::Left),
        Operator::new(Rule::logical_and, Assoc::Left),
        Operator::new(Rule::equal, Assoc::Left) | Operator::new(Rule::not_equal, Assoc::Left),
        Operator::new(Rule::less_than, Assoc::Left)
            | Operator::new(Rule::less_equal, Assoc::Left)
            | Operator::new(Rule::greater_than, Assoc::Left)
            | Operator::new(Rule::greater_equal, Assoc::Left),
        Operator::new(Rule::plus, Assoc::Left) | Operator::new(Rule::minus, Assoc::Left),
        Operator::new(Rule::multiply, Assoc::Left)
            | Operator::new(Rule::divide, Assoc::Left)
            | Operator::new(Rule::modulo, Assoc::Left),
        Operator::new(Rule::exponent, Assoc::Right),
    ])
}

#[derive(Parser)]
#[grammar = "grammar/yolk.pest"]
pub struct YolkParser;

/// Parses Yolk statements from source text.
pub fn parse(source: &str) -> Result<Vec<YolkNode>, ParseError> {
    let mut ast = vec![];
    let pairs = YolkParser::parse(Rule::program, source)
        .map_err(|e| ParseError::BadSyntax(e.to_string()))?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::import_stmt => ast.push(parse_import_stmt(pair)),
            Rule::define_stmt => ast.push(parse_define_stmt(pair)),
            Rule::let_stmt => ast.push(parse_let_stmt(pair)),
            Rule::export_stmt => ast.push(parse_export_stmt(pair)),
            Rule::comment => (),
            Rule::EOI => (),
            _ => panic!("expected rule statement, but got: {:?}", pair),
        }
    }
    Ok(ast)
}

fn parse_import_stmt(stmt: Pair<Rule>) -> YolkNode {
    let mut pairs = stmt.into_inner();
    let ident = pairs.next().expect("failed to unwrap ident from pair");
    YolkNode::ImportStmt {
        ident: ident.as_str().to_string(),
    }
}

fn parse_define_stmt(stmt: Pair<Rule>) -> YolkNode {
    let mut pairs = stmt.into_inner();
    let ident = pairs.next().expect("failed to unwrap ident from pair");
    let params = pairs.next().expect("failed to unwrap params from pair");
    let body = pairs.next().expect("failed to unwrap body from pair");
    YolkNode::DefineStmt {
        ident: ident.as_str().to_string(),
        params: params
            .into_inner()
            .map(|x| x.as_str().to_string())
            .collect(),
        body: Box::new(parse_expr(body)),
    }
}

fn parse_let_stmt(stmt: Pair<Rule>) -> YolkNode {
    let mut pairs = stmt.into_inner();
    let ident = pairs.next().expect("failed to unwrap ident from pair");
    let expr = pairs.next().expect("failed to unwrap expr from pair");
    YolkNode::LetStmt {
        ident: ident.as_str().to_string(),
        expr: Box::new(parse_expr(expr)),
    }
}

fn parse_export_stmt(stmt: Pair<Rule>) -> YolkNode {
    let mut pairs = stmt.into_inner();
    let ident = pairs.next().expect("failed to unwrap ident from pair");
    YolkNode::ExportStmt {
        ident: ident.as_str().to_string(),
    }
}

fn parse_expr(expr: Pair<Rule>) -> YolkNode {
    match expr.as_rule() {
        Rule::prefix_expr => {
            let mut pairs = expr.into_inner();
            let op = pairs.next().expect("failed to unwrap op from pair");
            let expr = pairs.next().expect("failed to unwrap expr from pair");
            YolkNode::PrefixExpr {
                op: match op.as_rule() {
                    Rule::logical_not => PrefixOp::Not,
                    Rule::abs => PrefixOp::Abs,
                    Rule::sqrt => PrefixOp::Sqrt,
                    Rule::sin => PrefixOp::Sin,
                    Rule::cos => PrefixOp::Cos,
                    Rule::tan => PrefixOp::Tan,
                    Rule::asin => PrefixOp::Asin,
                    Rule::acos => PrefixOp::Acos,
                    Rule::atan => PrefixOp::Atan,
                    _ => panic!("expected prefix op, but got: {:?}", op),
                },
                expr: Box::new(parse_expr(expr)),
            }
        }
        Rule::builtin_expr => {
            let mut pairs = expr.into_inner();
            let ident = pairs.next().expect("failed to unwrap ident from pair");
            let args = pairs.next().expect("failed to unwrap args from pair");
            YolkNode::BuiltinExpr {
                ident: ident.as_str().to_string(),
                args: args.into_inner().map(parse_expr).collect(),
            }
        }
        Rule::call_expr => {
            let mut pairs = expr.into_inner();
            let ident = pairs.next().expect("failed to unwrap ident from pair");
            let args = pairs.next().expect("failed to unwrap args from pair");
            YolkNode::CallExpr {
                ident: ident.as_str().to_string(),
                args: args.into_inner().map(parse_expr).collect(),
            }
        }
        Rule::infix_expr => PREC_CLIMBER.climb(
            expr.into_inner(),
            |pair: Pair<Rule>| parse_expr(pair),
            |lhs: YolkNode, op: Pair<Rule>, rhs: YolkNode| YolkNode::InfixExpr {
                lhs: Box::new(lhs),
                op: match op.as_rule() {
                    Rule::plus => InfixOp::Add,
                    Rule::minus => InfixOp::Sub,
                    Rule::multiply => InfixOp::Mul,
                    Rule::divide => InfixOp::Div,
                    Rule::modulo => InfixOp::Mod,
                    Rule::exponent => InfixOp::Exp,
                    Rule::less_than => InfixOp::LessThan,
                    Rule::less_equal => InfixOp::LessEqual,
                    Rule::greater_than => InfixOp::GreaterThan,
                    Rule::greater_equal => InfixOp::GreaterEqual,
                    Rule::equal => InfixOp::Equal,
                    Rule::not_equal => InfixOp::NotEqual,
                    Rule::logical_and => InfixOp::And,
                    Rule::logical_or => InfixOp::Or,
                    _ => panic!("expected infix op, but got: {:?}", op),
                },
                rhs: Box::new(rhs),
            },
        ),
        Rule::ident => YolkNode::Ident(expr.as_str().to_string()),
        Rule::literal => {
            //TODO: handle error better
            YolkNode::Literal(YololNumber::from_str(expr.as_str()).unwrap_or(YololNumber::zero()))
        }
        Rule::array => {
            let exprs: Vec<YolkNode> = expr.into_inner().map(parse_expr).collect();
            YolkNode::Array(exprs)
        }
        _ => panic!("expected rule expression, but got: {:?}", expr),
    }
}
