use crate::ast::YololNode;

#[cfg(test)]
mod tests;

const LINE_LIMIT: usize = 70;

const IDENT_PREC: u32 = 1000;
const LIT_PREC: u32 = 1000;

/// Formats Yolol statements as a program.
///
/// Lines will be limited to LINE_LIMIT when possible. Whitespace will be minimized.
///
/// # Panics
///
/// Panics if any of the nodes are not statements, or if any of the nodes are malformed.
pub fn format_as_program(stmts: &[YololNode]) -> String {
    let mut program = String::new();
    let mut line = String::new();
    for stmt in stmts.iter() {
        if let YololNode::AssignStmt { ident, expr } = stmt {
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
        } else {
            panic!("expected Yolol assign statement, but got: {:?}", stmt)
        }
    }
    // Add final line and trim whitespace
    program.push_str(line.as_str());
    program.trim().to_string()
}

fn format_expr(expr: &YololNode, parent_prec: u32) -> (String, u32) {
    match expr {
        YololNode::PrefixExpr { op, expr } => {
            let prec = op.to_precedence();
            let (expr, child_prec) = format_expr(expr, prec);
            let add_parens = prec < parent_prec;
            let add_space = prec <= child_prec;
            let formatted = format!(
                "{lparen}{op}{space}{expr}{rparen}",
                lparen = if add_parens { "(" } else { "" },
                op = op.to_string(),
                space = if add_space { " " } else { "" },
                expr = expr,
                rparen = if add_parens { ")" } else { "" },
            );
            (formatted, prec)
        }
        YololNode::InfixExpr { lhs, op, rhs } => {
            let is_alpha = op.to_string().chars().all(char::is_alphabetic);
            let prec = op.to_precedence();
            let (lhs, lhs_prec) = format_expr(lhs, prec);
            let (rhs, rhs_prec) = format_expr(rhs, prec);
            // If the operation is commutative, we can omit parentheses in the case of matching
            // precedence. This prevents "a-(b-c)" from being formatted as "a-b-c".
            let add_parens = if op.is_commutative() {
                prec < parent_prec
            } else {
                prec <= parent_prec
            };
            let add_lhs_space = (prec <= lhs_prec) && is_alpha;
            let add_rhs_space = (prec <= rhs_prec) && is_alpha;
            let formatted = format!(
                "{lparen}{lhs}{lhs_space}{op}{rhs_space}{rhs}{rparen}",
                lparen = if add_parens { "(" } else { "" },
                lhs = lhs,
                lhs_space = if add_lhs_space { " " } else { "" },
                op = op.to_string(),
                rhs_space = if add_rhs_space { " " } else { "" },
                rhs = rhs,
                rparen = if add_parens { ")" } else { "" },
            );
            (formatted, prec)
        }
        YololNode::Ident(s) => (s.to_string(), IDENT_PREC),
        YololNode::Literal(y) => (y.to_string(), LIT_PREC),
        _ => panic!("expected Yolol expression, but got {:?}", expr),
    }
}
