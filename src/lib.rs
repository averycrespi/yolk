#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod ast;
pub mod environment;
pub mod error;
pub mod format;
pub mod function;
pub mod optimizer;
pub mod parser;
pub mod transpiler;
pub mod value;

pub use format::format_as_program;
pub use optimizer::optimize;
pub use parser::parse;
pub use transpiler::transpile;
