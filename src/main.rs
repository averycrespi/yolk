extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::error::Error;
use pest::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "yolk.pest"]
pub struct YolkParser;

#[derive(Debug)]
pub enum AstNode {
    LetStmt {
        ident: String,
        expr: Box<AstNode>,
    },
    UnaryExpr {
        op: UnaryOp,
        expr: Box<AstNode>,
    },
    BinaryExpr {
        lhs: Box<AstNode>,
        op: BinaryOp,
        rhs: Box<AstNode>,
    },
    Ident(String),
    Number(f64),
    Array(Vec<AstNode>),
}

#[derive(Debug)]
pub enum UnaryOp {
    Not,
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
}

pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = vec![];
    let stmts = YolkParser::parse(Rule::program, source)?;
    for stmt in stmts {
        match stmt.as_rule() {
            Rule::let_stmt => ast.push(parse_let_stmt(stmt)),
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    Ok(ast)
}

fn parse_let_stmt(stmt: pest::iterators::Pair<Rule>) -> AstNode {
    let mut pair = stmt.into_inner();
    let ident = pair.next().unwrap();
    let expr = pair.next().unwrap();
    AstNode::LetStmt {
        ident: String::from(ident.as_str()),
        expr: Box::new(parse_expr(expr)),
    }
}

fn parse_expr(expr: pest::iterators::Pair<Rule>) -> AstNode {
    let expr = expr.into_inner().next().unwrap();
    match expr.as_rule() {
        Rule::unary_expr => {
            let mut pair = expr.into_inner();
            let op = pair.next().unwrap();
            let expr = pair.next().unwrap();
            parse_unary_expr(op, parse_expr(expr))
        }
        Rule::binary_expr => {
            let mut pair = expr.into_inner();
            let lhs = pair.next().unwrap();
            let op = pair.next().unwrap();
            let rhs = pair.next().unwrap();
            parse_binary_expr(parse_expr(lhs), op, parse_expr(rhs))
        }
        Rule::base_expr => parse_base_expr(expr),
        _ => unreachable!(),
    }
}

fn parse_unary_expr(op: pest::iterators::Pair<Rule>, expr: AstNode) -> AstNode {
    AstNode::UnaryExpr {
        op: match op.as_str() {
            "not" => UnaryOp::Not,
            _ => unreachable!(),
        },
        expr: Box::new(expr),
    }
}

fn parse_binary_expr(lhs: AstNode, op: pest::iterators::Pair<Rule>, rhs: AstNode) -> AstNode {
    AstNode::BinaryExpr {
        lhs: Box::new(lhs),
        op: match op.as_str() {
            "+" => BinaryOp::Add,
            _ => unreachable!(),
        },
        rhs: Box::new(rhs),
    }
}

fn parse_base_expr(expr: pest::iterators::Pair<Rule>) -> AstNode {
    let expr = expr.into_inner().next().unwrap();
    match expr.as_rule() {
        Rule::ident => {
            let mut pair = expr.into_inner();
            let ident = pair.next().unwrap();
            AstNode::Ident(String::from(ident.as_str()))
        }
        Rule::number => {
            let mut pair = expr.into_inner();
            let number = pair.next().unwrap();
            let float: f64 = number.as_str().parse().unwrap();
            AstNode::Number(float)
        }
        Rule::array => {
            let exprs: Vec<AstNode> = expr.into_inner().map(parse_expr).collect();
            AstNode::Array(exprs)
        }
        _ => unreachable!(),
    }
}

fn main() {
    let source = fs::read_to_string("example.yolk").expect("cannot read file");
    let ast = parse(&source).expect("failed parse");
    println!("{:?}", ast);
}
