//! # Searching strategies

use std::collections::HashMap;

use crate::Node;

pub mod astar;
pub mod greedy;

pub enum Strategies {
    ASTAR,
    GREEDY,
}

impl ToString for Strategies {
    fn to_string(&self) -> String {
        use Strategies::*;

        match self {
            ASTAR => "A star",
            GREEDY => "Greedy",
        }
        .to_owned()
    }
}

pub trait Searcher {
    fn search(
        state_space_graph: &HashMap<&'static str, Node>,
        initial_state: &'static str,
        goal_state: &'static str,
    ) -> Option<Vec<&'static str>>;
}

fn heuristic(a: &str, nodes: &HashMap<&str, Node>) -> i32 {
    if let Some(node) = nodes.get(a) {
        return node.heuristic_value;
    }

    0
}
