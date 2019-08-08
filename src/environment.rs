use crate::ast::YololNode;
use crate::error::YolkError;
use crate::function::Function;
use crate::value::{ArrayExpr, NumberExpr, Value};

use std::collections::{HashMap, HashSet};

/// Represents a Yolk program environment.
#[derive(Debug, Clone)]
pub struct Environment {
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
    // Stores the identifiers of reserved builtins
    builtins: HashSet<String>,
    // Stores variable identifiers that must not be eliminated
    saved: HashSet<String>,
}

impl Environment {
    /// Creates an empty environment.
    pub fn new() -> Environment {
        Environment {
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
            ]
            .iter()
            .cloned()
            .collect(),
            builtins: ["sum".to_string(), "product".to_string()]
                .iter()
                .cloned()
                .collect(),
            saved: HashSet::new(),
        }
    }

    /// Gets the value of a variable from an environment.
    pub fn variable(&self, ident: &str) -> Result<Value, YolkError> {
        let ident = ident.to_string();
        match self.variables.get(&ident) {
            Some(value) => Ok(value.clone()),
            None => Err(YolkError::GetUndefinedVariable(ident)),
        }
    }

    /// Gets a function from an environment.
    pub fn function(&self, ident: &str) -> Result<Function, YolkError> {
        let ident = ident.to_string();
        match self.functions.get(&ident) {
            Some(function) => Ok(function.clone()),
            None => Err(YolkError::GetUndefinedFunction(ident)),
        }
    }

    /// Returs the variable identifiers that must not be eliminated.
    pub fn saved(&self) -> HashSet<String> {
        self.saved.clone()
    }

    /// Imports a variable into an environment.
    pub fn import(&mut self, ident: &str) -> Result<(), YolkError> {
        let ident = ident.to_string();
        if self.imports.contains(&ident) {
            Err(YolkError::ImportTwice(ident))
        } else if self.variables.contains_key(&ident) {
            Err(YolkError::ImportExisting(ident))
        } else if self.keywords.contains(&ident) {
            Err(YolkError::ImportKeyword(ident))
        } else {
            self.imports.insert(ident.clone());
            self.variables
                .insert(ident.clone(), Value::Number(NumberExpr::from_ident(&ident)));
            Ok(())
        }
    }

    /// Defines a function in an environmnent.
    pub fn define(&mut self, ident: &str, function: &Function) -> Result<(), YolkError> {
        let ident = ident.to_string();
        if self.functions.contains_key(&ident) {
            Err(YolkError::RedefineFunction(ident))
        } else if self.builtins.contains(&ident) {
            Err(YolkError::DefineBuiltin(ident))
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
            Err(YolkError::ReassignVariable(ident))
        } else if self.keywords.contains(&ident) {
            Err(YolkError::AssignToKeyword(ident))
        } else if self
            .variables
            .iter()
            .map(|(s, _)| s.to_lowercase())
            .collect::<Vec<String>>()
            .contains(&ident.to_lowercase())
        {
            Err(YolkError::AssignInsensitive(ident))
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
    pub fn export(&mut self, ident: &str) -> Result<(), YolkError> {
        let ident = ident.to_string();
        if self.exports.contains(&ident) {
            Err(YolkError::ExportTwice(ident))
        } else {
            match self.variables.get(&ident) {
                Some(Value::Number(number)) => match number.as_expr() {
                    YololNode::Ident(s) => {
                        self.saved.insert(s.to_string());
                    }
                    _ => panic!("expected identifier, but got: {:?}", number.as_expr()),
                },
                Some(Value::Array(array)) => {
                    for expr in array.as_exprs().iter() {
                        match expr {
                            YololNode::Ident(s) => {
                                self.saved.insert(s.to_string());
                            }
                            _ => panic!("expected identifier, but got: {:?}", expr),
                        }
                    }
                }
                None => return Err(YolkError::GetUndefinedVariable(ident)),
            }
            self.exports.insert(ident);
            Ok(())
        }
    }
}
