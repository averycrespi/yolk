use crate::error::YolkError;

#[derive(Debug, Clone)]
pub struct Environment {
    imports: Vec<String>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            imports: Vec::new(),
        }
    }

    pub fn import(&mut self, ident: &str) -> Result<(), YolkError> {
        let ident = ident.to_string();
        if self.imports.contains(&ident) {
            return Err(YolkError::ExistingImport { ident: ident });
        }
        self.imports.push(ident);
        Ok(())
    }
}
