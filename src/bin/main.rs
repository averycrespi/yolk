use yolk::parser::parse;
use yolk::transpiler::transpile;

use std::fs;

fn main() {
    let source = fs::read_to_string("example.yolk").expect("cannot read file");
    println!("{:?}", parse(&source));
}
