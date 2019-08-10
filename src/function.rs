use std::collections::HashSet;

use crate::ast::YolkNode;
use crate::error::YolkError;

/// Represents a Yolk function.
#[derive(Debug, Clone)]
pub struct Function {
    ident: String,
    params: Vec<String>,
    body: YolkNode,
}

impl Function {
    /// Creates a new Yolk function.
    pub fn new(ident: &str, params: &[String], body: &YolkNode) -> Result<Function, YolkError> {
        let function = Function {
            ident: ident.to_string(),
            params: params.to_vec(),
            body: body.clone(),
        };
        function.check_for_duplicate_params()?;
        function.check_body_node(&function.body)?;
        Ok(function)
    }

    fn check_for_duplicate_params(&self) -> Result<(), YolkError> {
        let mut uniq = HashSet::new();
        if self.params.iter().all(move |x| uniq.insert(x)) {
            Ok(())
        } else {
            Err(YolkError::DuplicateParams(self.ident.to_string()))
        }
    }

    fn check_body_node(&self, node: &YolkNode) -> Result<(), YolkError> {
        match node {
            YolkNode::PrefixExpr { op: _, expr } => self.check_body_node(expr)?,
            YolkNode::BuiltinExpr { ident, args } | YolkNode::CallExpr { ident, args } => {
                for arg in args.iter() {
                    self.check_body_node(arg)?;
                }
                // Check for recursive calls
                if self.ident == ident.to_string() {
                    return Err(YolkError::RecursiveCall(self.ident.to_string()));
                }
            }
            YolkNode::InfixExpr { lhs, op: _, rhs } => {
                self.check_body_node(lhs)?;
                self.check_body_node(rhs)?;
            }
            YolkNode::Ident(s) => {
                // Check for undefined local variables
                if !self.params.contains(s) {
                    return Err(YolkError::GetUndefinedLocal {
                        function: self.ident.to_string(),
                        local: s.to_string(),
                    });
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
    pub fn call(&self, args: &[YolkNode]) -> Result<YolkNode, YolkError> {
        if self.params.len() != args.len() {
            Err(YolkError::WrongNumberOfArgs(self.ident.to_string()))
        } else {
            self.replace_params_with_args(args, &self.body)
        }
    }

    fn replace_params_with_args(
        &self,
        args: &[YolkNode],
        node: &YolkNode,
    ) -> Result<YolkNode, YolkError> {
        match node {
            YolkNode::PrefixExpr { op, expr } => Ok(YolkNode::PrefixExpr {
                op: *op,
                expr: Box::new(self.replace_params_with_args(args, expr)?),
            }),
            YolkNode::BuiltinExpr {
                ident,
                args: call_args,
            } => {
                let mut replaced_args = Vec::new();
                for arg in call_args.iter() {
                    replaced_args.push(self.replace_params_with_args(args, arg)?);
                }
                Ok(YolkNode::BuiltinExpr {
                    ident: ident.to_string(),
                    args: replaced_args,
                })
            }
            YolkNode::CallExpr {
                ident,
                args: call_args,
            } => {
                let mut replaced_args = Vec::new();
                for arg in call_args.iter() {
                    replaced_args.push(self.replace_params_with_args(args, arg)?);
                }
                Ok(YolkNode::CallExpr {
                    ident: ident.to_string(),
                    args: replaced_args,
                })
            }
            YolkNode::InfixExpr { lhs, op, rhs } => Ok(YolkNode::InfixExpr {
                lhs: Box::new(self.replace_params_with_args(args, lhs)?),
                op: *op,
                rhs: Box::new(self.replace_params_with_args(args, rhs)?),
            }),
            // Replace local variables with their respective arguments
            YolkNode::Ident(s) => {
                let index = self
                    .params
                    .iter()
                    .position(|param| param == s)
                    .expect("failed to get index of param");
                Ok(args[index].clone())
            }
            YolkNode::Array(exprs) => {
                let mut replaced_exprs = Vec::new();
                for expr in exprs.iter() {
                    replaced_exprs.push(self.replace_params_with_args(args, expr)?);
                }
                Ok(YolkNode::Array(replaced_exprs))
            }
            _ => Ok(node.clone()),
        }
    }
}
