use std::collections::{HashMap, HashSet};

use crate::ast::{YololExpr, YololStmt};
use crate::error::TranspileError;
use crate::function::Function;
use crate::value::{ArrayExpr, NumberExpr, Value};

#[cfg(test)]
mod tests;

/// Represents contextual information about a Yolk program.
///
/// Contexts are created by the transpiler and given to the optimizer.
#[derive(Debug, Clone)]
pub struct Context {
    exported: HashSet<String>,
}

impl Context {
    fn new() -> Context {
        Context {
            exported: HashSet::new(),
        }
    }

    fn push_exported(&mut self, ident: &str) {
        self.exported.insert(ident.to_string());
    }

    /// Returns a set of exported Yolol identifiers.
    pub fn exported(&self) -> HashSet<String> {
        self.exported.clone()
    }
}

/// Represents a Yolk program environment.
#[derive(Debug, Clone)]
pub struct Environment {
    context: Context,
    // Stores the identifiers of imported variables
    imports: HashSet<String>,
    // Maps variable identifiers to values
    variables: HashMap<String, Value>,
    // Maps function identifiers to functions
    functions: HashMap<String, Function>,
    // Stores the identifiers of exported variables
    exports: HashSet<String>,
    // Stores the identifiers of reserved keywords
    keywords: HashSet<String>,
}

impl Environment {
    /// Creates an empty environment.
    pub fn new() -> Environment {
        Environment {
            context: Context::new(),
            imports: HashSet::new(),
            variables: HashMap::new(),
            functions: HashMap::new(),
            exports: HashSet::new(),
            keywords: [
                "import".to_string(),
                "define".to_string(),
                "let".to_string(),
                "export".to_string(),
                "not".to_string(),
                "abs".to_string(),
                "sqrt".to_string(),
                "sin".to_string(),
                "cos".to_string(),
                "tan".to_string(),
                "asin".to_string(),
                "acos".to_string(),
                "atan".to_string(),
                "and".to_string(),
                "or".to_string(),
                "sum".to_string(),
                "product".to_string(),
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }

    /// Gets the context of an environment.
    pub fn context(&self) -> Context {
        self.context.clone()
    }

    /// Gets the value of a variable from an environment.
    pub fn variable(&self, ident: &str) -> Result<Value, TranspileError> {
        match self.variables.get(ident) {
            Some(value) => Ok(value.clone()),
            None => Err(TranspileError::GetUndefinedVariable(ident.to_string())),
        }
    }

    /// Gets a function from an environment.
    pub fn function(&self, ident: &str) -> Result<Function, TranspileError> {
        match self.functions.get(ident) {
            Some(function) => Ok(function.clone()),
            None => Err(TranspileError::GetUndefinedFunction(ident.to_string())),
        }
    }

    /// Imports a variable into an environment.
    pub fn import(&mut self, ident: &str) -> Result<(), TranspileError> {
        if self.imports.contains(ident) {
            Err(TranspileError::ImportTwice(ident.to_string()))
        } else if self.variables.contains_key(ident) {
            Err(TranspileError::ImportExisting(ident.to_string()))
        } else if self.keywords.contains(ident) {
            Err(TranspileError::ImportKeyword(ident.to_string()))
        } else {
            self.imports.insert(ident.to_string());
            self.variables.insert(
                ident.to_string(),
                Value::Number(NumberExpr::from_ident(ident)),
            );
            Ok(())
        }
    }

    /// Defines a function in an environmnent.
    pub fn define(&mut self, ident: &str, function: Function) -> Result<(), TranspileError> {
        if self.functions.contains_key(ident) {
            Err(TranspileError::RedefineFunction(ident.to_string()))
        } else if self.keywords.contains(ident) {
            Err(TranspileError::DefineKeyword(ident.to_string()))
        } else {
            self.functions.insert(ident.to_string(), function);
            Ok(())
        }
    }

    /// Assigns a value to a variable in an environment.
    ///
    /// Returns the associated Yolol assign statements.
    pub fn let_value(
        &mut self,
        ident: &str,
        value: Value,
    ) -> Result<Vec<YololStmt>, TranspileError> {
        if self.imports.contains(ident) || self.variables.contains_key(ident) {
            Err(TranspileError::ReassignVariable(ident.to_string()))
        } else if self.keywords.contains(ident) {
            Err(TranspileError::AssignToKeyword(ident.to_string()))
        } else if self
            .variables
            .iter()
            .map(|(s, _)| s.to_lowercase())
            .collect::<Vec<String>>()
            .contains(&ident.to_lowercase())
        {
            Err(TranspileError::AssignSameLowercase(ident.to_string()))
        } else {
            match value {
                Value::Number(number) => {
                    let assign_stmt = number.to_assign_stmt(&ident);
                    self.variables.insert(
                        ident.to_string(),
                        Value::Number(NumberExpr::from_ident(&ident)),
                    );
                    Ok(vec![assign_stmt])
                }
                Value::Array(array) => {
                    let assign_stmts = array.to_assign_stmts(&ident);
                    self.variables.insert(
                        ident.to_string(),
                        Value::Array(ArrayExpr::from_ident(&ident, assign_stmts.len())),
                    );
                    Ok(assign_stmts)
                }
            }
        }
    }

    /// Exports a variable from an environment.
    ///
    /// Tracks which variable identifiers must not be eliminated.
    pub fn export(&mut self, ident: &str) -> Result<(), TranspileError> {
        if self.exports.contains(ident) {
            Err(TranspileError::ExportTwice(ident.to_string()))
        } else {
            match self.variables.get(ident) {
                Some(Value::Number(number)) => match number.as_expr() {
                    YololExpr::Ident(s) => {
                        self.context.push_exported(&s);
                    }
                    _ => panic!("expected identifier, but got: {:?}", number.as_expr()),
                },
                Some(Value::Array(array)) => {
                    for expr in array.as_exprs().iter() {
                        match expr {
                            YololExpr::Ident(s) => {
                                self.context.push_exported(&s);
                            }
                            _ => panic!("expected identifier, but got: {:?}", expr),
                        }
                    }
                }
                None => return Err(TranspileError::ExportUndefined(ident.to_string())),
            }
            self.exports.insert(ident.to_string());
            Ok(())
        }
    }
}
