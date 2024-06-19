//! # Informed search algorithm

#![allow(unused)]

use std::{cmp::Ordering, collections::HashMap};

use search_strategies::Searcher;

use crate::search_strategies::{astar::AStar, greedy::Greedy, Strategies};

mod search_strategies;

pub enum SearchError {
    NoPathFound,
}

pub struct Node {
    state: &'static str,
    heuristic_value: i32,
    arc: Vec<(&'static str, i32)>,
}

impl Node {
    fn new(state: &'static str, heuristic_value: i32, arc: Vec<(&'static str, i32)>) -> Self {
        Node {
            state,
            heuristic_value,
            arc,
        }
    }
}


#[derive(Copy, Clone, Eq, PartialEq)]
struct SearchNode<'a> {
    state: &'a str,
    g: Option<i32>,
    h: i32,
}

impl<'a> SearchNode<'a> {
    fn f(&self) -> i32 {
        if let Some(g) = self.g {
            return g + self.h
        }

        0
    }
}

impl<'a> Ord for SearchNode<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f().cmp(&self.f())
            .then_with(|| other.h.cmp(&self.h))
            .then_with(|| self.state.cmp(&other.state))
    }
}

impl<'a> PartialOrd for SearchNode<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct InformedSearch {
    state_space_graph: HashMap<&'static str, Node>,
    initial_state: &'static str,
    goal_state: &'static str,
}

impl InformedSearch {
    pub fn new(
        state_space_graph: HashMap<&'static str, Node>,
        initial_state: &'static str,
        goal_state: &'static str,
    ) -> Self {
        Self {
            state_space_graph,
            initial_state,
            goal_state,
        }
    }

    pub fn search(&self, strategy: Strategies) -> Result<Vec<&str>, SearchError> {
        use Strategies::*;

        let path = match strategy {
            ASTAR => AStar::search(&self.state_space_graph, self.initial_state, self.goal_state),
            GREEDY => Greedy::search(&self.state_space_graph, self.initial_state, self.goal_state),
        };

        if path.is_none() {
            return Err(SearchError::NoPathFound);
        }

        Ok(path.unwrap())
    }
}

pub fn pretty_print_path(path: &[&str]) {
    todo!()
}
