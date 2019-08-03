use pest::error::Error;
use pest::Parser;

use crate::ast::{InfixOp, PrefixOp, YolkNode as Node};

#[derive(Parser)]
#[grammar = "grammar/yolk.pest"]
pub struct YolkParser;

pub fn parse(source: &str) -> Result<Vec<Node>, Error<Rule>> {
    let mut ast = vec![];
    let pairs = YolkParser::parse(Rule::program, source)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::import_stmt => ast.push(parse_import_stmt(pair)),
            Rule::define_stmt => ast.push(parse_define_stmt(pair)),
            Rule::let_stmt => ast.push(parse_let_stmt(pair)),
            Rule::export_stmt => ast.push(parse_export_stmt(pair)),
            Rule::EOI => (),
            _ => panic!("unexpected pair: {:?}", pair),
        }
    }
    Ok(ast)
}

fn parse_import_stmt(stmt: pest::iterators::Pair<Rule>) -> Node {
    let mut pair = stmt.into_inner();
    let ident = pair.next().unwrap();
    Node::ImportStmt {
        ident: String::from(ident.as_str()),
    }
}

fn parse_define_stmt(stmt: pest::iterators::Pair<Rule>) -> Node {
    let mut pair = stmt.into_inner();
    let ident = pair.next().unwrap();
    let params = pair.next().unwrap();
    let body = pair.next().unwrap();
    Node::DefineStmt {
        ident: String::from(ident.as_str()),
        params: params
            .into_inner()
            .map(|x| String::from(x.as_str()))
            .collect(),
        body: Box::new(parse_expr(body)),
    }
}

fn parse_let_stmt(stmt: pest::iterators::Pair<Rule>) -> Node {
    let mut pair = stmt.into_inner();
    let ident = pair.next().unwrap();
    let expr = pair.next().unwrap();
    Node::LetStmt {
        ident: String::from(ident.as_str()),
        expr: Box::new(parse_expr(expr)),
    }
}

fn parse_export_stmt(stmt: pest::iterators::Pair<Rule>) -> Node {
    let mut pair = stmt.into_inner();
    let ident = pair.next().unwrap();
    Node::ExportStmt {
        ident: String::from(ident.as_str()),
    }
}

fn parse_expr(expr: pest::iterators::Pair<Rule>) -> Node {
    match expr.as_rule() {
        Rule::prefix_expr => {
            let mut pair = expr.into_inner();
            let op = pair.next().unwrap();
            let expr = pair.next().unwrap();
            parse_prefix_expr(op, parse_expr(expr))
        }
        Rule::macro_expr => {
            let mut pair = expr.into_inner();
            let ident = pair.next().unwrap();
            let args = pair.next().unwrap();
            Node::MacroExpr {
                ident: String::from(ident.as_str()),
                args: args.into_inner().map(parse_expr).collect(),
            }
        }
        Rule::infix_expr => {
            let mut pair = expr.into_inner();
            let lhs = pair.next().unwrap();
            let op = pair.next().unwrap();
            let rhs = pair.next().unwrap();
            parse_infix_expr(parse_expr(lhs), op, parse_expr(rhs))
        }
        Rule::ident => Node::Ident(String::from(expr.as_str())),
        Rule::number => {
            let float: f64 = expr.as_str().parse().unwrap();
            Node::Number(float)
        }
        Rule::array => {
            let exprs: Vec<Node> = expr.into_inner().map(parse_expr).collect();
            Node::Array(exprs)
        }
        _ => panic!("unexpected expression: {:?}", expr),
    }
}

fn parse_prefix_expr(op: pest::iterators::Pair<Rule>, expr: Node) -> Node {
    Node::PrefixExpr {
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
            _ => panic!("unexpected prefix op: {}", op.as_str()),
        },
        expr: Box::new(expr),
    }
}

fn parse_infix_expr(lhs: Node, op: pest::iterators::Pair<Rule>, rhs: Node) -> Node {
    Node::InfixExpr {
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
            ":" => InfixOp::Join,
            _ => panic!("unexpected infix op: {}", op.as_str()),
        },
        rhs: Box::new(rhs),
    }
}
