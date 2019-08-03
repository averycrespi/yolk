use yolk::parser::parse;

use std::fs;

fn main() {
    let source = fs::read_to_string("example.yolk").expect("cannot read file");
    let ast = parse(&source).expect("failed parse");
    println!("{:?}", ast);
}
