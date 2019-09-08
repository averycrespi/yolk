//! Numerical computing for Yolol
//!
//! Yolk is a domain-specific language that transpiles to Yolol.
//!
//! # Quick Start
//!
//! ```
//! // Parse a Yolk program from a string
//! let yolk: YolkProgram = "let foo = 1".parse().unwrap();
//!
//! // Transpile a Yolk program to Yolol, then optimize
//! let yolol: YololProgram = yolk.try_into().unwrap().optimize();
//!
//! // Print a Yolol program as a chip
//! println!("{}", yolol.to_string());
//! ```

#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod ast;
pub mod error;
pub mod optimizer;
pub mod parser;
pub mod transpiler;

pub use ast::{YolkProgram, YololProgram};
pub use error::YolkError;
