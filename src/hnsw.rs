//hnsw.rs

use rand::Rng;

pub struct Node {
    pub id: u64,
    pub vector: Vec<f32>,
    pub level: usize,
    pub neighbors: Vec<Vec<usize>>,
}

pub fn hnsw() {}

pub fn random_level() -> usize {
    let p = 0.5;
    let mut rng = rand::thread_rng();
    let mut level = 0;
    while rng.r#gen::<f64>() < p {
        level += 1;
        if level == 16 {
            break;
        }
    }
    level
}
