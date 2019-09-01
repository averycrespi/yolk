use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use crate::ast::{YololExpr, YololStmt};

/// Represents a DAG of dependencies between Yolol variables.
#[derive(Debug, Clone)]
pub struct DepGraph {
    graph: HashMap<String, HashSet<String>>,
}

impl DepGraph {
    /// Creates a dependency graph from Yolol statements.
    ///
    /// # Panics
    ///
    /// Panic if any of the nodes are not statements, or if any of the nodes are malformed.
    pub fn from_statements(stmts: &[YololStmt]) -> DepGraph {
        let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
        for stmt in stmts.iter() {
            match stmt {
                YololStmt::Assign { ident, expr } => {
                    let mut deps = HashSet::new();
                    DepGraph::find_deps(&mut deps, expr);
                    graph.insert(ident.to_string(), deps);
                }
            }
        }
        DepGraph { graph: graph }
    }

    /// Search for dependent variables from a starting set of identifiers.
    pub fn search_from(&self, idents: &HashSet<String>) -> HashSet<String> {
        let mut found = HashSet::new();
        let mut queue = Vec::from_iter(idents.iter().cloned());
        while queue.len() > 0 {
            let ident = queue.pop().expect("failed to pop ident from queue");
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

    fn find_deps(deps: &mut HashSet<String>, expr: &YololExpr) {
        match expr {
            YololExpr::Prefix { op: _, expr } => DepGraph::find_deps(deps, expr),
            YololExpr::Infix { lhs, op: _, rhs } => {
                DepGraph::find_deps(deps, lhs);
                DepGraph::find_deps(deps, rhs);
            }
            YololExpr::Ident(s) => {
                deps.insert(s.to_string());
            }
            _ => (),
        }
    }
}
