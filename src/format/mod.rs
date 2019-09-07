use crate::ast::YololStmt;

#[cfg(test)]
mod tests;

const LINE_LIMIT: usize = 70;

/// Formats Yolol statements as a program.
///
/// Lines will be limited to LINE_LIMIT when possible. Whitespace will be minimized.
///
/// # Panics
///
/// Panics if any of the nodes are not statements, or if any of the nodes are malformed.
pub fn format_as_program(stmts: &[YololStmt]) -> String {
    let mut program = String::new();
    let mut line = String::new();
    for stmt in stmts.iter() {
        match stmt {
            YololStmt::Assign { ident, expr } => {
                let formatted = expr.to_string();
                if line.len() + formatted.len() + 1 > LINE_LIMIT {
                    program.push_str(&format!(
                        "{line}\n{ident}={expr}\n",
                        line = line,
                        ident = ident,
                        expr = formatted
                    ));
                    line.clear();
                } else {
                    line.push_str(&format!("{ident}={expr} ", ident = ident, expr = formatted));
                }
            }
        }
    }
    // Add final line and trim whitespace
    program.push_str(line.as_str());
    program.trim().to_string()
}
