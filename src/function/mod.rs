use std::collections::HashSet;

use crate::ast::YolkNode;
use crate::error::TranspileError;

#[cfg(test)]
mod tests;

/// Represents a Yolk function.
#[derive(Debug, Clone)]
pub struct Function {
    ident: String,
    params: Vec<String>,
    body: YolkNode,
}

impl Function {
    /// Creates a new Yolk function.
    pub fn new(
        ident: &str,
        params: &[String],
        body: &YolkNode,
    ) -> Result<Function, TranspileError> {
        if params.len() < 1 {
            panic!("function has no parameters");
        }
        let function = Function {
            ident: ident.to_string(),
            params: params.to_vec(),
            body: body.clone(),
        };
        function.check_for_duplicate_params()?;
        function.check_body_node(&function.body)?;
        Ok(function)
    }

    fn check_for_duplicate_params(&self) -> Result<(), TranspileError> {
        let mut uniq = HashSet::new();
        if self.params.iter().all(move |x| uniq.insert(x)) {
            Ok(())
        } else {
            Err(TranspileError::DuplicateParams)
        }
    }

    fn check_body_node(&self, node: &YolkNode) -> Result<(), TranspileError> {
        match node {
            YolkNode::PrefixExpr { op: _, expr } => self.check_body_node(expr)?,
            YolkNode::BuiltinExpr { ident, args } | YolkNode::CallExpr { ident, args } => {
                for arg in args.iter() {
                    self.check_body_node(arg)?;
                }
                // Check for recursive calls
                if self.ident == ident.to_string() {
                    return Err(TranspileError::RecursiveCall);
                }
            }
            YolkNode::InfixExpr { lhs, op: _, rhs } => {
                self.check_body_node(lhs)?;
                self.check_body_node(rhs)?;
            }
            YolkNode::Ident(s) => {
                // Check for undefined local variables
                if !self.params.contains(s) {
                    return Err(TranspileError::GetUndefinedLocal(s.to_string()));
                }
            }
            YolkNode::Array(exprs) => {
                for expr in exprs.iter() {
                    self.check_body_node(expr)?;
                }
            }
            _ => (),
        }
        Ok(())
    }

    /// Calls a function with arguments.
    pub fn call(&self, args: &[YolkNode]) -> Result<YolkNode, TranspileError> {
        if self.params.len() != args.len() {
            Err(TranspileError::WrongNumberOfArgs(self.ident.to_string()))
        } else {
            Ok(self.replace_params_with_args(args, &self.body))
        }
    }

    fn replace_params_with_args(&self, args: &[YolkNode], node: &YolkNode) -> YolkNode {
        match node {
            YolkNode::PrefixExpr { op, expr } => YolkNode::PrefixExpr {
                op: *op,
                expr: Box::new(self.replace_params_with_args(args, expr)),
            },
            YolkNode::BuiltinExpr {
                ident,
                args: call_args,
            } => {
                let mut replaced_args = Vec::new();
                for arg in call_args.iter() {
                    replaced_args.push(self.replace_params_with_args(args, arg));
                }
                YolkNode::BuiltinExpr {
                    ident: ident.to_string(),
                    args: replaced_args,
                }
            }
            YolkNode::CallExpr {
                ident,
                args: call_args,
            } => {
                let mut replaced_args = Vec::new();
                for arg in call_args.iter() {
                    replaced_args.push(self.replace_params_with_args(args, arg));
                }
                YolkNode::CallExpr {
                    ident: ident.to_string(),
                    args: replaced_args,
                }
            }
            YolkNode::InfixExpr { lhs, op, rhs } => YolkNode::InfixExpr {
                lhs: Box::new(self.replace_params_with_args(args, lhs)),
                op: *op,
                rhs: Box::new(self.replace_params_with_args(args, rhs)),
            },
            // Replace local variables with their respective arguments
            YolkNode::Ident(s) => {
                let index = self
                    .params
                    .iter()
                    .position(|param| param == s)
                    .expect("failed to get index of param");
                args[index].clone()
            }
            YolkNode::Array(exprs) => {
                let mut replaced_exprs = Vec::new();
                for expr in exprs.iter() {
                    replaced_exprs.push(self.replace_params_with_args(args, expr));
                }
                YolkNode::Array(replaced_exprs)
            }
            _ => node.clone(),
        }
    }
}
