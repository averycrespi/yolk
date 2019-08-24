use crate::ast::YololNode;

const LINE_LIMIT: usize = 70;
const NO_PREC: u32 = 0;

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
            let formatted = format_expr(&expr, NO_PREC);
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

fn format_expr(expr: &YololNode, parent_prec: u32) -> String {
    match expr {
        YololNode::PrefixExpr { op, expr } => {
            let prec = op.to_precedence();
            let use_parens = prec < parent_prec;
            format!(
                "{lparen}{op}{space}{expr}{rparen}",
                lparen = if use_parens { "(" } else { "" },
                op = op.to_string(),
                //TODO: remove space when not needed
                space = " ",
                expr = format_expr(expr, prec),
                rparen = if use_parens { ")" } else { "" },
            )
        }
        YololNode::InfixExpr { lhs, op, rhs } => {
            let prec = op.to_precedence();
            let use_parens = prec < parent_prec;
            format!(
                "{lparen}{lhs}{lspace}{op}{rspace}{rhs}{rparen}",
                lparen = if use_parens { "(" } else { "" },
                lhs = format_expr(lhs, prec),
                //TODO: remove space when not needed
                lspace = " ",
                op = op.to_string(),
                //TODO: remove space when not needed
                rspace = " ",
                rhs = format_expr(rhs, prec),
                rparen = if use_parens { ")" } else { "" },
            )
        }
        YololNode::Ident(s) => s.to_string(),
        YololNode::Literal(y) => y.to_string(),
        _ => panic!("expected Yolol expression, but got {:?}", expr),
    }
}
