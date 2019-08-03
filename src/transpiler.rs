use crate::array::Array;
use crate::ast::{InfixOp, PrefixOp, YolkNode, YololNode};
use crate::environment::Env;
use crate::number::Number;

pub fn transpile(stmts: Vec<YolkNode>) -> Result<Vec<YololNode>, &'static str> {
    Err("not implemented")
}
