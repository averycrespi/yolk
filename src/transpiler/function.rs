//TODO: refactor

use std::collections::HashSet;

use crate::ast::YolkExpr;
use crate::error::YolkError;

/// Represents a Yolk function.
#[derive(Debug, Clone)]
pub struct Function {
    ident: String,
    params: Vec<String>,
    body: YolkExpr,
}

impl Function {
    /// Creates a new Yolk function.
    pub fn new(ident: &str, params: &[String], body: &YolkExpr) -> Result<Function, YolkError> {
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

    fn check_for_duplicate_params(&self) -> Result<(), YolkError> {
        let mut uniq = HashSet::new();
        if self.params.iter().all(move |x| uniq.insert(x)) {
            Ok(())
        } else {
            Err(YolkError::DuplicateParams {
                func: self.ident.to_string(),
            })
        }
    }

    fn check_body_node(&self, node: &YolkExpr) -> Result<(), YolkError> {
        match node {
            YolkExpr::Prefix { op: _, expr } => self.check_body_node(expr)?,
            YolkExpr::Fold { op: _, args } => {
                for arg in args.iter() {
                    self.check_body_node(arg)?;
                }
            }
            YolkExpr::Call { ident, args } => {
                for arg in args.iter() {
                    self.check_body_node(arg)?;
                }
                // Check for recursive calls
                if self.ident == ident.to_string() {
                    return Err(YolkError::RecursiveCall {
                        func: self.ident.to_string(),
                    });
                }
            }
            YolkExpr::Infix { lhs, op: _, rhs } => {
                self.check_body_node(lhs)?;
                self.check_body_node(rhs)?;
            }
            YolkExpr::Ident(s) => {
                // Check for undefined local variables
                if !self.params.contains(s) {
                    return Err(YolkError::UndefinedVariable { var: s.to_string() });
                }
            }
            YolkExpr::Array(exprs) => {
                for expr in exprs.iter() {
                    self.check_body_node(expr)?;
                }
            }
            _ => (),
        }
        Ok(())
    }

    /// Calls a function with arguments.
    pub fn call(&self, args: &[YolkExpr]) -> Result<YolkExpr, YolkError> {
        if self.params.len() != args.len() {
            Err(YolkError::WrongNumberOfArgs {
                func: self.ident.to_string(),
            })
        } else {
            Ok(self.replace_params_with_args(args, &self.body))
        }
    }

    fn replace_params_with_args(&self, args: &[YolkExpr], node: &YolkExpr) -> YolkExpr {
        match node {
            YolkExpr::Prefix { op, expr } => YolkExpr::Prefix {
                op: *op,
                expr: Box::new(self.replace_params_with_args(args, expr)),
            },
            YolkExpr::Fold {
                op,
                args: call_args,
            } => {
                let mut replaced_args = Vec::new();
                for arg in call_args.iter() {
                    replaced_args.push(self.replace_params_with_args(args, arg));
                }
                YolkExpr::Fold {
                    op: *op,
                    args: replaced_args,
                }
            }
            YolkExpr::Call {
                ident,
                args: call_args,
            } => {
                let mut replaced_args = Vec::new();
                for arg in call_args.iter() {
                    replaced_args.push(self.replace_params_with_args(args, arg));
                }
                YolkExpr::Call {
                    ident: ident.to_string(),
                    args: replaced_args,
                }
            }
            YolkExpr::Infix { lhs, op, rhs } => YolkExpr::Infix {
                lhs: Box::new(self.replace_params_with_args(args, lhs)),
                op: *op,
                rhs: Box::new(self.replace_params_with_args(args, rhs)),
            },
            // Replace local variables with their respective arguments
            YolkExpr::Ident(s) => {
                let index = self
                    .params
                    .iter()
                    .position(|param| param == s)
                    .expect("failed to get index of param");
                args[index].clone()
            }
            YolkExpr::Array(exprs) => {
                let mut replaced_exprs = Vec::new();
                for expr in exprs.iter() {
                    replaced_exprs.push(self.replace_params_with_args(args, expr));
                }
                YolkExpr::Array(replaced_exprs)
            }
            _ => node.clone(),
        }
    }
}
