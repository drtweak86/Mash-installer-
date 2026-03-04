use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};

/// A simple dependency graph for installer phases.
pub struct DependencyGraph {
    nodes: HashMap<String, HashSet<String>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, name: String, dependencies: Vec<String>) {
        self.nodes.insert(name, dependencies.into_iter().collect());
    }

    /// Performs a topological sort on the graph.
    /// Returns a list of names in an order that satisfies all dependencies.
    pub fn topological_sort(&self) -> Result<Vec<String>> {
        let mut sorted = Vec::new();
        let mut visited = HashSet::new();
        let mut visiting = HashSet::new();

        for name in self.nodes.keys() {
            self.visit(name, &mut visited, &mut visiting, &mut sorted)?;
        }

        Ok(sorted)
    }

    fn visit(
        &self,
        name: &str,
        visited: &mut HashSet<String>,
        visiting: &mut HashSet<String>,
        sorted: &mut Vec<String>,
    ) -> Result<()> {
        if visited.contains(name) {
            return Ok(());
        }

        if visiting.contains(name) {
            return Err(anyhow!(
                "Circular dependency detected involving phase: {}",
                name
            ));
        }

        visiting.insert(name.to_string());

        if let Some(deps) = self.nodes.get(name) {
            for dep in deps {
                // Only visit dependency if it exists in the graph.
                // If it doesn't exist, it might be a phase that was filtered out (should_run = false).
                // We'll handle missing but required dependencies elsewhere or just skip them here.
                if self.nodes.contains_key(dep) {
                    self.visit(dep, visited, visiting, sorted)?;
                }
            }
        }

        visiting.remove(name);
        visited.insert(name.to_string());
        sorted.push(name.to_string());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topological_sort() -> Result<()> {
        let mut graph = DependencyGraph::new();
        graph.add_node("A".into(), vec![]);
        graph.add_node("B".into(), vec!["A".into()]);
        graph.add_node("C".into(), vec!["B".into()]);
        graph.add_node("D".into(), vec!["A".into(), "C".into()]);

        let sorted = graph.topological_sort()?;

        let a_pos = sorted.iter().position(|x| x == "A").unwrap();
        let b_pos = sorted.iter().position(|x| x == "B").unwrap();
        let c_pos = sorted.iter().position(|x| x == "C").unwrap();
        let d_pos = sorted.iter().position(|x| x == "D").unwrap();

        assert!(a_pos < b_pos);
        assert!(b_pos < c_pos);
        assert!(c_pos < d_pos);

        Ok(())
    }

    #[test]
    fn test_circular_dependency() {
        let mut graph = DependencyGraph::new();
        graph.add_node("A".into(), vec!["B".into()]);
        graph.add_node("B".into(), vec!["A".into()]);

        let result = graph.topological_sort();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Circular dependency"));
    }

    #[test]
    fn test_missing_dependency_is_ignored() -> Result<()> {
        let mut graph = DependencyGraph::new();
        graph.add_node("B".into(), vec!["A".into()]);

        let sorted = graph.topological_sort()?;
        assert_eq!(sorted, vec!["B"]);
        Ok(())
    }
}
