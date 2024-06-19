//! # A* search

use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::{Node, SearchNode};

use super::{heuristic, Searcher};

pub struct AStar;

impl Searcher for AStar {
    fn search(
        state_space_graph: &HashMap<&'static str, Node>,
        initial_state: &'static str,
        goal_state: &'static str,
    ) -> Option<Vec<&'static str>> {
        let mut open_list = BinaryHeap::new();
        let mut closed_list = HashSet::new();
        let mut came_from: HashMap<&str, &str> = HashMap::new();

        let start_node = SearchNode {
            state: initial_state,
            g: Some(0),
            h: heuristic(initial_state, state_space_graph),
        };

        open_list.push(start_node);

        while let Some(current_node) = open_list.pop() {
            if current_node.state == goal_state {
                let mut path = Vec::new();
                let mut current = current_node.state;
                while let Some(&parent) = came_from.get(current) {
                    path.push(current);
                    current = parent;
                }
                path.push(initial_state);
                path.reverse();
                return Some(path);
            }

            closed_list.insert(current_node.state);

            if let Some(node) = state_space_graph.get(current_node.state) {
                for &next_state in &node.arc {
                    if closed_list.contains(next_state) {
                        continue;
                    }

                    let tentative_g = match current_node.g {
                        Some(g) => g + node.backward_cost,
                        _ => 0
                    };
                    let neighbor_node = SearchNode {
                        state: next_state,
                        g: Some(tentative_g),
                        h: heuristic(next_state, state_space_graph),
                    };

                    if !open_list
                        .iter()
                        .any(|&n| n.state == neighbor_node.state && tentative_g >= n.g.unwrap())
                    {
                        open_list.push(neighbor_node);
                        came_from.insert(next_state, current_node.state);
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::Node;

    use super::{AStar, Searcher};

    #[test]
    fn astar_search() {
        let mut state_space_graph = HashMap::from([
            ("A", Node::new("A", 7, 1, vec!["B", "C"])),
            ("B", Node::new("B", 6, 1, vec!["D", "E"])),
            ("C", Node::new("C", 2, 1, vec!["F"])),
            ("D", Node::new("D", 5, 1, vec![])),
            ("E", Node::new("E", 3, 1, vec!["G"])),
            ("F", Node::new("F", 1, 1, vec!["G"])),
            ("G", Node::new("G", 0, 1, vec![])),
        ]);

    
        let initial_state = "A";
        let goal_state = "G";

        let path = AStar::search(&state_space_graph, initial_state, goal_state);

        assert!(path.is_some());

        println!("Found path : {:?}", path.unwrap());
    }
}