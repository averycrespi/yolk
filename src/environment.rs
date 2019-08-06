use crate::ast::YololNode;
use crate::error::YolkError;
use crate::function::Function;
use crate::value::{Array, Number, Value};

use std::collections::HashMap;

/// Represents a Yolk program environment.
#[derive(Debug, Clone)]
pub struct Environment {
    // Stores the identifiers of imported variables
    imports: Vec<String>,
    // Maps variable identifiers to values
    variables: HashMap<String, Value>,
    // Maps function identifiers to functions
    functions: HashMap<String, Function>,
    // Stores the identifiers of exported variables
    exports: Vec<String>,
    // Stores the identifiers of reserved keywords
    keywords: Vec<String>,
    // Stores the identifiers of reserved builtins
    builtins: Vec<String>,
}

impl Environment {
    /// Creates an empty environment.
    pub fn new() -> Environment {
        Environment {
            imports: Vec::new(),
            variables: HashMap::new(),
            functions: HashMap::new(),
            exports: Vec::new(),
            keywords: vec![
                "import".to_string(),
                "define".to_string(),
                "let".to_string(),
                "export".to_string(),
            ],
            builtins: vec!["sum".to_string(), "product".to_string()],
        }
    }

    /// Gets the value of a variable from an environment.
    pub fn variable(&self, ident: &str) -> Result<Value, YolkError> {
        let ident = ident.to_string();
        match self.variables.get(&ident) {
            Some(value) => Ok(value.clone()),
            None => Err(YolkError::UndefinedVariable(ident)),
        }
    }

    /// Gets a function from an environment.
    pub fn function(&self, ident: &str) -> Result<Function, YolkError> {
        let ident = ident.to_string();
        match self.functions.get(&ident) {
            Some(function) => Ok(function.clone()),
            None => Err(YolkError::UndefinedFunction(ident)),
        }
    }

    /// Imports a variable into an environment.
    pub fn import(&mut self, ident: &str) -> Result<(), YolkError> {
        let ident = ident.to_string();
        if self.imports.contains(&ident) {
            Err(YolkError::DuplicateImport(ident))
        } else if self.variables.contains_key(&ident) {
            Err(YolkError::ExistingImport(ident))
        } else if self.keywords.contains(&ident) {
            Err(YolkError::ReservedKeyword(ident))
        } else {
            self.imports.push(ident.clone());
            self.variables
                .insert(ident.clone(), Value::Number(Number::from_ident(&ident)));
            Ok(())
        }
    }

    /// Defines a function in an environmnent.
    pub fn define(&mut self, ident: &str, function: &Function) -> Result<(), YolkError> {
        let ident = ident.to_string();
        if self.functions.contains_key(&ident) {
            Err(YolkError::ExistingFunction(ident))
        } else if self.builtins.contains(&ident) {
            Err(YolkError::ReservedBuiltin(ident))
        } else {
            self.functions.insert(ident, function.to_owned());
            Ok(())
        }
    }

    /// Assigns a value to a variable in an environment.
    ///
    /// Returns the associated Yolol assign statements.
    pub fn let_value(&mut self, ident: &str, value: &Value) -> Result<Vec<YololNode>, YolkError> {
        let ident = ident.to_string();
        if self.imports.contains(&ident) || self.variables.contains_key(&ident) {
            Err(YolkError::ExistingVariable(ident))
        } else if self.keywords.contains(&ident) {
            Err(YolkError::ReservedKeyword(ident))
        } else if self
            .variables
            .iter()
            .map(|(s, _)| s.to_lowercase())
            .collect::<Vec<String>>()
            .contains(&ident.to_lowercase())
        {
            Err(YolkError::ConflictingVariable(ident))
        } else {
            match value {
                Value::Number(number) => {
                    let assign_stmt = number.to_assign_stmt(&ident);
                    self.variables
                        .insert(ident.to_string(), Value::Number(Number::from_ident(&ident)));
                    Ok(vec![assign_stmt])
                }
                Value::Array(array) => {
                    let assign_stmts = array.to_assign_stmts(&ident);
                    self.variables.insert(
                        ident.to_string(),
                        Value::Array(Array::from_ident(&ident, assign_stmts.len())),
                    );
                    Ok(assign_stmts)
                }
            }
        }
    }

    /// Exports a variable from an environment.
    pub fn export(&mut self, ident: &str) -> Result<(), YolkError> {
        let ident = ident.to_string();
        if self.exports.contains(&ident) {
            Err(YolkError::DuplicateExport(ident))
        } else if self.imports.contains(&ident) {
            Err(YolkError::ImportedExport(ident))
        } else if !self.variables.contains_key(&ident) {
            Err(YolkError::UndefinedVariable(ident))
        } else {
            self.exports.push(ident);
            Ok(())
        }
    }
}
