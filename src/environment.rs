use crate::ast::YololNode;
use crate::error::YolkError;
use crate::function::Function;
use crate::value::Value;

use std::collections::HashMap;

/// Represents a Yolk program environment.
#[derive(Debug, Clone)]
pub struct Environment {
    imports: Vec<String>,
    variables: HashMap<String, Value>,
    functions: HashMap<String, Function>,
    exports: Vec<String>,
    number_index: u32,
    array_index: u32,
}

impl Environment {
    /// Creates an empty environment.
    pub fn new() -> Environment {
        Environment {
            imports: Vec::new(),
            variables: HashMap::new(),
            functions: HashMap::new(),
            exports: Vec::new(),
            number_index: 0,
            array_index: 0,
        }
    }

    /// Imports a variable into an environment.
    pub fn import(&mut self, ident: &str) -> Result<(), YolkError> {
        let ident = ident.to_string();
        if self.imports.contains(&ident) {
            return Err(YolkError::DuplicateImport { ident: ident });
        } else if self.variables.contains_key(&ident) {
            return Err(YolkError::ExistingImport { ident: ident });
        }
        self.imports.push(ident);
        Ok(())
    }

    /// Defines a function in an environmnent.
    pub fn define(&mut self, ident: &str, function: &Function) -> Result<(), YolkError> {
        let ident = ident.to_string();
        if self.functions.contains_key(&ident) {
            return Err(YolkError::ExistingFunction { ident: ident });
        }
        self.functions.insert(ident, function.to_owned());
        Ok(())
    }

    /// Assigns a value to a variable in an environment.
    ///
    /// Returns the associated Yolol assign statements.
    pub fn let_value(&mut self, ident: &str, value: &Value) -> Result<Vec<YololNode>, YolkError> {
        let ident = ident.to_string();
        if self.imports.contains(&ident) || self.variables.contains_key(&ident) {
            return Err(YolkError::ExistingVariable { ident: ident });
        }
        match value {
            Value::Number(number) => {
                let (number, assign) = number.resolve(self.number_index);
                self.number_index += 1;
                self.variables.insert(ident, Value::Number(number));
                Ok(vec![assign])
            }
            Value::Array(array) => {
                let (array, assigns) = array.resolve(self.array_index);
                self.array_index += 1;
                self.variables.insert(ident, Value::Array(array));
                Ok(assigns)
            }
        }
    }

    /// Exports a variable from an environment.
    pub fn export(&mut self, ident: &str) -> Result<(), YolkError> {
        let ident = ident.to_string();
        if self.exports.contains(&ident) {
            return Err(YolkError::DuplicateExport { ident: ident });
        } else if !self.variables.contains_key(&ident) {
            return Err(YolkError::UndefinedExport { ident: ident });
        }
        self.exports.push(ident);
        Ok(())
    }
}
