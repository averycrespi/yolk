use crate::ast::{YolkNode, YololNode};
use crate::environment::Environment;
use crate::error::YolkError;

pub fn transpile(stmts: Vec<YolkNode>) -> Result<Vec<YololNode>, YolkError> {
    let mut env = Environment::new();
    let mut nodes = Vec::new();
    for stmt in stmts {
        match stmt {
            YolkNode::ImportStmt { ident } => transpile_import_stmt(&mut env, &ident)?,
            YolkNode::DefineStmt {
                ident,
                params,
                body,
            } => transpile_define_stmt(&mut env, &ident, params, &*body)?,
            _ => panic!("unexpected statement: {:?}", stmt),
        }
    }
    Ok(nodes)
}

fn transpile_import_stmt(env: &mut Environment, ident: &str) -> Result<(), YolkError> {
    env.import(ident)
}

fn transpile_define_stmt(
    env: &mut Environment,
    ident: &str,
    params: Vec<String>,
    body: &YolkNode,
) -> Result<(), YolkError> {
    //TODO
    Ok(())
}
