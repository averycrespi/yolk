use crate::ast::YolkNode;
use crate::error::YolkError;

use std::collections::HashSet;

/// Represents a Yolk function.
#[derive(Debug, Clone)]
pub struct Function {
    params: Vec<String>,
    body: YolkNode,
}

impl Function {
    /// Creates a new Yolk function.
    pub fn new(ident: &str, params: Vec<String>, body: &YolkNode) -> Result<Function, YolkError> {
        Function::check_params(ident, params.clone())?;
        Function::check_calls(ident, body)?;
        Function::check_locals(&ident, params.clone(), body)?;
        Ok(Function {
            params: params.clone(),
            body: body.clone(),
        })
    }

    fn check_params(ident: &str, params: Vec<String>) -> Result<(), YolkError> {
        let mut set = HashSet::new();
        let is_unique = params.iter().all(move |x| set.insert(x));
        if is_unique {
            Ok(())
        } else {
            Err(YolkError::DuplicateParams(ident.to_string()))
        }
    }

    fn check_calls(ident: &str, body: &YolkNode) -> Result<(), YolkError> {
        let mut calls = Vec::new();
        body.find(&mut calls, &|n| match n {
            YolkNode::CallExpr { ident: _, args: _ } => true,
            _ => false,
        });
        for call in calls.iter() {
            if let YolkNode::CallExpr {
                ident: call_ident,
                args: _,
            } = call
            {
                if ident == call_ident {
                    return Err(YolkError::RecursiveCall(ident.to_string()));
                }
            }
        }
        Ok(())
    }

    fn check_locals(ident: &str, params: Vec<String>, body: &YolkNode) -> Result<(), YolkError> {
        let mut locals = Vec::new();
        body.find(&mut locals, &|n| match n {
            YolkNode::Ident(_) => true,
            _ => false,
        });
        for local in locals.iter() {
            if let YolkNode::Ident(local_ident) = local {
                if !params.contains(local_ident) {
                    return Err(YolkError::UndefinedLocalVariable {
                        function: ident.to_string(),
                        variable: local_ident.to_string(),
                    });
                }
            }
        }
        Ok(())
    }

    /// Calls a function with arguments.
    pub fn call(&self, args: Vec<YolkNode>) -> Result<YolkNode, YolkError> {
        //TODO: implement
        Err(YolkError::NotImplemented)
    }
}
