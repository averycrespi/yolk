use yolk::optimizer::optimize;
use yolk::parser::parse;
use yolk::transpiler::transpile;

use std::fs;

fn main() {
    let source = fs::read_to_string("example.yolk").expect("cannot read file");
    let yolk = parse(&source).unwrap_or_else(|e| panic!("{}", e));
    let yolol = transpile(&yolk).unwrap_or_else(|e| panic!("{}", e));
    println!("raw:\n");
    for stmt in yolol.iter() {
        println!("{}", stmt);
    }
    let optimized = optimize(&yolol);
    println!("\noptimized:\n");
    for stmt in optimized.iter() {
        println!("{}", stmt);
    }
}
