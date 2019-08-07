use crate::ast::YololNode;

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

/// Represents dependencies between Yolol variables.
#[derive(Debug, Clone)]
pub struct DepGraph {
    graph: HashMap<String, HashSet<String>>,
}

impl DepGraph {
    /// Creates a dependency graph from Yolol assign statements.
    ///
    /// # Panics
    ///
    /// Panics if any of the statements are malformed.
    pub fn from_assign_stmts(stmts: &[YololNode]) -> DepGraph {
        let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
        for stmt in stmts.iter() {
            if let YololNode::AssignStmt { ident, expr } = stmt {
                let mut deps = HashSet::new();
                DepGraph::find_deps(&mut deps, expr);
                graph.insert(ident.to_string(), deps);
            } else {
                panic!("expected assign statement, but got: {:?}", stmt)
            }
        }
        DepGraph { graph: graph }
    }

    /// Search for connected variables from a starting set.
    pub fn search_from(&self, idents: &HashSet<String>) -> HashSet<String> {
        let mut found = HashSet::new();
        let mut queue = Vec::from_iter(idents.iter().cloned());
        while queue.len() > 0 {
            let ident = queue.pop().unwrap();
            if !found.contains(&ident) {
                found.insert(ident.to_string());
                if let Some(deps) = self.graph.get(&ident) {
                    for dep in deps.iter() {
                        queue.push(dep.to_string());
                    }
                }
            }
        }
        found
    }

    fn find_deps(deps: &mut HashSet<String>, expr: &YololNode) {
        match expr {
            YololNode::PrefixExpr { op: _, expr } => DepGraph::find_deps(deps, expr),
            YololNode::InfixExpr { lhs, op: _, rhs } => {
                DepGraph::find_deps(deps, lhs);
                DepGraph::find_deps(deps, rhs);
            }
            YololNode::Ident(s) => {
                deps.insert(s.to_string());
            }
            _ => (),
        }
    }
}
