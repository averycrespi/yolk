use crate::ast::{YololExpr, YololStmt};

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
                let (formatted, _) = format_expr(&expr, 0);
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

type HasParens = bool;

fn format_expr(expr: &YololExpr, parent_prec: u32) -> (String, HasParens) {
    match expr {
        YololExpr::Prefix { op, expr } => {
            let prec = op.to_precedence();
            let (expr, child_has_parens) = format_expr(expr, prec);
            let has_parens = prec < parent_prec;
            let has_space = !child_has_parens;
            let formatted = format!(
                "{lparen}{op}{space}{expr}{rparen}",
                lparen = if has_parens { "(" } else { "" },
                op = op.to_string(),
                space = if has_space { " " } else { "" },
                expr = expr,
                rparen = if has_parens { ")" } else { "" },
            );
            (formatted, has_parens)
        }
        YololExpr::Infix { lhs, op, rhs } => {
            let is_alpha = op.to_string().chars().all(char::is_alphabetic);
            let prec = op.to_precedence();
            let (lhs, lhs_has_parens) = format_expr(lhs, prec);
            let (rhs, rhs_has_parens) = format_expr(rhs, prec);
            // If the operation is associative, we can format a+(b+c) as (a+b)+c,
            // then omit the parentheses to get a+b+c.
            let has_parens = if op.is_associative() {
                prec < parent_prec
            } else {
                prec <= parent_prec
            };
            let has_lhs_space = !lhs_has_parens && is_alpha;
            let has_rhs_space = !rhs_has_parens && is_alpha;
            let formatted = format!(
                "{lparen}{lhs}{lhs_space}{op}{rhs_space}{rhs}{rparen}",
                lparen = if has_parens { "(" } else { "" },
                lhs = lhs,
                lhs_space = if has_lhs_space { " " } else { "" },
                op = op.to_string(),
                rhs_space = if has_rhs_space { " " } else { "" },
                rhs = rhs,
                rparen = if has_parens { ")" } else { "" },
            );
            (formatted, has_parens)
        }
        YololExpr::Ident(s) => (s.to_string(), false),
        YololExpr::Literal(y) => (y.to_string(), false),
    }
}
