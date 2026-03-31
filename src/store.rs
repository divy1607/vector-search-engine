use crate::hnsw::Node;
pub struct Store {
    pub dim: usize,
    pub data: Vec<Node>,
    pub next_id: u64,
}

impl Store {
    pub fn new() -> Store {
        Store {
            dim: 0,
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

    // pub fn insert(&mut self, vec: Vec<f32>) -> Result<u64, String> {
    //     if self.data.is_empty() {
    //         self.dim = vec.len();
    //     } else {
    //         if vec.len() != self.dim {
    //             return Err("dimension mismatch".to_string());
    //         }
    //     }
    //     let id = self.next_id;
    //     self.next_id += 1;
    //     self.data.push((id, vec));

    //     Ok(id)
    // }
}

