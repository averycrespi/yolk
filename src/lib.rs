#[macro_use]
extern crate lazy_static;
extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod ast;
pub mod environment;
pub mod error;
pub mod function;
pub mod graph;
pub mod optimizer;
pub mod parser;
pub mod transpiler;
pub mod value;

pub use optimizer::optimize;
pub use parser::parse;
pub use transpiler::transpile;
