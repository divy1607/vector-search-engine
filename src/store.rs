use crate::hnsw::Node;
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

    pub fn insert(&self, vector: Vec<f32>) {

    }
}

