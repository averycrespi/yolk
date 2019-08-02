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
    PrefixExpr {
        op: PrefixOp,
        expr: Box<AstNode>,
    },
    MacroExpr {
        ident: String,
        args: Vec<AstNode>,
    },
    InfixExpr {
        lhs: Box<AstNode>,
        op: InfixOp,
        rhs: Box<AstNode>,
    },
    Ident(String),
    Number(f64),
    Array(Vec<AstNode>),
}

#[derive(Debug)]
pub enum PrefixOp {
    Not,
    Abs,
    Sqrt,
    Sin,
    Cos,
    Tan,
    Arcsin,
    Arccos,
    Arctan,
}

#[derive(Debug)]
pub enum InfixOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Exp,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    Equal,
    NotEqual,
    And,
    Or,
    Join,
}

pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = vec![];
    let stmts = YolkParser::parse(Rule::program, source)?;
    for stmt in stmts {
        match stmt.as_rule() {
            Rule::let_stmt => ast.push(parse_let_stmt(stmt)),
            Rule::EOI => (),
            _ => panic!("unexpected statement: {:?}", stmt),
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
            let args: Vec<AstNode> = pair.map(parse_expr).collect();
            AstNode::MacroExpr {
                ident: String::from(ident.as_str()),
                args: args,
            }
        }
        Rule::infix_expr => {
            let mut pair = expr.into_inner();
            let lhs = pair.next().unwrap();
            let op = pair.next().unwrap();
            let rhs = pair.next().unwrap();
            parse_infix_expr(parse_expr(lhs), op, parse_expr(rhs))
        }
        Rule::ident => AstNode::Ident(String::from(expr.as_str())),
        Rule::number => {
            let float: f64 = expr.as_str().parse().unwrap();
            AstNode::Number(float)
        }
        Rule::array => {
            let exprs: Vec<AstNode> = expr.into_inner().map(parse_expr).collect();
            AstNode::Array(exprs)
        }
        _ => panic!("unexpected expression: {:?}", expr),
    }
}

fn parse_prefix_expr(op: pest::iterators::Pair<Rule>, expr: AstNode) -> AstNode {
    AstNode::PrefixExpr {
        op: match op.as_str() {
            "not" => PrefixOp::Not,
            "abs" => PrefixOp::Abs,
            "sqrt" => PrefixOp::Sqrt,
            "sin" => PrefixOp::Sin,
            "cos" => PrefixOp::Cos,
            "tan" => PrefixOp::Tan,
            "arcsin" => PrefixOp::Arcsin,
            "arccos" => PrefixOp::Arccos,
            "arctan" => PrefixOp::Arctan,
            _ => panic!("unexpected prefix op: {}", op.as_str()),
        },
        expr: Box::new(expr),
    }
}

fn parse_infix_expr(lhs: AstNode, op: pest::iterators::Pair<Rule>, rhs: AstNode) -> AstNode {
    AstNode::InfixExpr {
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
            ":" => InfixOp::Join,
            _ => panic!("unexpected infix op: {}", op.as_str()),
        },
        rhs: Box::new(rhs),
    }
}

fn main() {
    let source = fs::read_to_string("example.yolk").expect("cannot read file");
    let ast = parse(&source).expect("failed parse");
    println!("{:?}", ast);
}
