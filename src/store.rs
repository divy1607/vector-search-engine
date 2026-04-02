// store.rs

use crate::distance::cosine_similarity;
use crate::hnsw::Node;
use crate::hnsw::random_level;
pub struct Store {
    pub nodes: Vec<Node>,
    pub entry_point: Option<usize>,
    pub max_level: usize,
}

impl Store {
    pub fn new() -> Store {
        Store {
            nodes: Vec::new(),
            data: Vec::new(),
            next_id: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn dim(&self) -> usize {
        self.dim
    }

    // pub fn iter(&self) -> std::slice::Iter<'_, (u64, Vec<f32>)> {
    //     self.data.iter()
    // }

    pub fn insert(&mut self, vector: Vec<f32>) {
        let level = random_level();
        let node_id = self.nodes.len();

        let mut neighbors: Vec<Vec<usize>> = Vec::new();
        for _ in 0..=level {
            neighbors.push(vec![]);
        }
        if self.entry_point.is_none() {
            let node = Node {
                id: node_id as u64,
                vector,
                level,
                neighbors,
            };
            self.nodes.push(node);
            self.entry_point = Some(node_id);
            self.max_level = level;
            return;
        }
        let mut current = self.entry_point.unwrap();
        for lvl in ((level + 1)..=self.max_level).rev() {
            current = self.greedy_search_at_level(current, &vector, lvl);
        }
        let node = Node {
            id: node_id as u64,
            vector,
            level,
            neighbors,
        };
        self.nodes.push(node);
        if level > self.max_level {
            self.entry_point = Some(node_id);
            self.max_level = level;
        }
    }

    pub fn greedy_search_at_level(&self, mut current: usize, query: &[f32], level: usize) -> usize {
        loop {
            let mut best = current;

            for &neighbor in &self.nodes[current].neighbors[level] {
                if cosine_similarity(query, &self.nodes[neighbor].vector)
                    > cosine_similarity(query, &self.nodes[best].vector)
                {
                    best = neighbor;
                }
            }

            if best == current {
                break;
            }

            current = best;
        }

        current
    }
}
