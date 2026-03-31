use rand::Rng;
pub struct Node {
    id: u64,
    vector: Vec<f32>,
    level: usize,
    neighbors: Vec<Vec<usize>>,
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
