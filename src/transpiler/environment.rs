use std::collections::{HashMap, HashSet};

use crate::ast::YololStmt;
use crate::error::YolkError;
use crate::transpiler::function::Function;
use crate::transpiler::value::{Value, Vector};

/// Represents a Yolk program environment.
#[derive(Debug, Clone)]
pub struct Environment {
    // Stores the identifiers of imported variables
    imports: HashSet<String>,
    // Maps variable identifiers to values
    variables: HashMap<String, Value>,
    // Stores the lowercase identifiers of variables
    // Used for detecting identifier conflicts
    lowercase: HashSet<String>,
    // Maps function identifiers to functions
    functions: HashMap<String, Function>,
    // Stores the identifiers of reserved keywords
    keywords: HashSet<String>,
}

impl Environment {
    /// Creates an empty environment.
    pub fn new() -> Environment {
        Environment {
            imports: HashSet::new(),
            variables: HashMap::new(),
            lowercase: HashSet::new(),
            functions: HashMap::new(),
            keywords: [
                "import".to_string(),
                "define".to_string(),
                "let".to_string(),
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

    /// Gets the value of a variable from an environment.
    pub fn variable(&self, ident: &str) -> Result<Value, YolkError> {
        match self.variables.get(ident) {
            Some(value) => Ok(value.clone()),
            None => Err(YolkError::UndefinedVariable {
                var: ident.to_string(),
            }),
        }
    }

    /// Gets a function from an environment.
    pub fn function(&self, ident: &str) -> Result<Function, YolkError> {
        match self.functions.get(ident) {
            Some(function) => Ok(function.clone()),
            None => Err(YolkError::UndefinedFunction {
                func: ident.to_string(),
            }),
        }
    }

    /// Imports a variable into an environment.
    pub fn import(&mut self, ident: &str) -> Result<(), YolkError> {
        if self.imports.contains(ident) | self.variables.contains_key(ident) {
            Err(YolkError::ImportExisting {
                var: ident.to_string(),
            })
        } else if self.keywords.contains(ident) {
            Err(YolkError::ImportKeyword {
                var: ident.to_string(),
            })
        } else {
            self.imports.insert(ident.to_string());
            self.variables
                .insert(ident.to_string(), Value::Scalar(ident.parse()?));
            self.lowercase.insert(ident.to_string().to_lowercase());
            Ok(())
        }
    }

    /// Defines a function in an environmnent.
    pub fn define(&mut self, ident: &str, function: Function) -> Result<(), YolkError> {
        if self.functions.contains_key(ident) {
            Err(YolkError::DefineExisting {
                func: ident.to_string(),
            })
        } else if self.keywords.contains(ident) {
            Err(YolkError::DefineKeyword {
                func: ident.to_string(),
            })
        } else {
            self.functions.insert(ident.to_string(), function);
            Ok(())
        }
    }

    /// Assigns a value to a variable in an environment.
    pub fn let_value(&mut self, ident: &str, value: Value) -> Result<Vec<YololStmt>, YolkError> {
        if self.imports.contains(ident) || self.variables.contains_key(ident) {
            Err(YolkError::AssignExisting {
                var: ident.to_string(),
            })
        } else if self.keywords.contains(ident) {
            Err(YolkError::AssignKeyword {
                var: ident.to_string(),
            })
        } else if self.lowercase.contains(&ident.to_lowercase()) {
            Err(YolkError::AssignConflict {
                var: ident.to_string(),
            })
        } else {
            match value {
                Value::Scalar(s) => {
                    let stmt = s.to_assign_stmt(&ident);
                    self.variables
                        .insert(ident.to_string(), Value::Scalar(ident.parse()?));
                    self.lowercase.insert(ident.to_string().to_lowercase());
                    Ok(vec![stmt])
                }
                Value::Vector(v) => {
                    let stmts = v.to_assign_stmts(&ident);
                    self.variables.insert(
                        ident.to_string(),
                        Value::Vector(Vector::from_expanded_ident(&ident, stmts.len())),
                    );
                    self.lowercase.insert(ident.to_string().to_lowercase());
                    Ok(stmts)
                }
            }
        }
    }
}
