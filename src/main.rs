extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast;
mod parser;

use std::fs;

fn main() {
    let source = fs::read_to_string("example.yolk").expect("cannot read file");
    let ast = parser::parse(&source).expect("failed parse");
    println!("{:?}", ast);
}
