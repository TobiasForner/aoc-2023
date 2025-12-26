use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
pub struct Graph<N> {
    pub edges: HashMap<N, Vec<(N, usize)>>,
}

impl<N: Hash + Eq + Clone + Debug> Graph<N> {
    pub fn new() -> Self {
        let edges = HashMap::new();
        Self { edges }
    }

    pub fn add_directed_edge(&mut self, start: N, end: N, weight: usize) {
        if let Some(vec) = self.edges.get_mut(&start) {
            vec.push((end, weight));
        } else {
            let vec = vec![(end, weight)];
            self.edges.insert(start, vec);
        }
    }
}

pub fn lcm(n1: usize, n2: usize) -> usize {
    let mut x;
    let mut y;
    if n1 > n2 {
        x = n1;
        y = n2;
    } else {
        x = n2;
        y = n1;
    }

    let mut rem = x % y;

    while rem != 0 {
        x = y;
        y = rem;
        rem = x % y;
    }

    let lcm = n1 * n2 / y;

    lcm
}
