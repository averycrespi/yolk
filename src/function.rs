use crate::ast::YolkNode;
use crate::error::YolkError;

use std::collections::HashSet;

/// Represents a Yolk function.
#[derive(Debug, Clone)]
pub struct Function {
    ident: String,
    params: Vec<String>,
    body: YolkNode,
}

impl Function {
    /// Creates a new Yolk function.
    pub fn new(ident: &str, params: Vec<String>, body: &YolkNode) -> Result<Function, YolkError> {
        let function = Function {
            ident: ident.to_string(),
            params: params.clone(),
            body: body.clone(),
        };
        function.check_params()?;
        function.check_node(&function.body)?;
        Ok(function)
    }

    fn check_params(&self) -> Result<(), YolkError> {
        let mut uniq = HashSet::new();
        if self.params.iter().all(move |x| uniq.insert(x)) {
            Ok(())
        } else {
            Err(YolkError::DuplicateParams(self.ident.to_string()))
        }
    }

    fn check_node(&self, node: &YolkNode) -> Result<(), YolkError> {
        match node {
            YolkNode::PrefixExpr { op: _, expr } => self.check_node(expr)?,
            YolkNode::CallExpr { ident: id, args } => {
                for arg in args.iter() {
                    self.check_node(arg)?;
                }
                if self.ident == id.to_string() {
                    return Err(YolkError::RecursiveCall(self.ident.to_string()));
                }
            }
            YolkNode::InfixExpr { lhs, op: _, rhs } => {
                self.check_node(lhs)?;
                self.check_node(rhs)?;
            }
            YolkNode::Ident(s) => {
                if !self.params.contains(s) {
                    return Err(YolkError::UndefinedLocalVariable {
                        function: self.ident.to_string(),
                        variable: s.to_string(),
                    });
                }
            }
            YolkNode::Array(exprs) => {
                for expr in exprs.iter() {
                    self.check_node(expr)?;
                }
            }
            _ => (),
        }
        Ok(())
    }

    /// Calls a function with arguments.
    pub fn call(&self, args: Vec<YolkNode>) -> Result<YolkNode, YolkError> {
        //TODO: implement
        Err(YolkError::NotImplemented)
    }
}
