use yolk::parser::parse;
use yolk::transpiler::transpile;

use std::fs;

fn main() {
    let source = fs::read_to_string("example.yolk").expect("cannot read file");
    let yolk = parse(&source).unwrap_or_else(|e| panic!("{}", e));
    println!("{:?}", yolk);
    let yolol = transpile(&yolk);
    println!("{:?}", yolol);
}
