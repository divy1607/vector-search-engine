//query.rs

use crate::distance::cosine_similarity;
use crate::store::{Store};
use crate::topk::top_k_ids;

pub fn compute_query(stores: &Store, query: &[f32], k: usize) -> Vec<(u64, f32)> {
    let mut scores: Vec<(u64, f32)> = Vec::new();
    for (id, vector) in stores.iter() {
        let score: f32 = cosine_similarity(vector.as_slice(), query);
        let tuple = (*id, score);
        scores.push(tuple);
    }

    let top_k_results = top_k_ids(scores, k);
    top_k_results
}
