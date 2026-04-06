//topk.rs

pub fn top_k_ids(mut data: Vec<(u64, f32)>, k: usize) -> Vec<(u64, f32)> {
    data.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
    data.iter().take(k).cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_ranking() {
        assert_eq!(top_k_ids(vec![(1, 0.9), (2, 0.8), (3, 0.95)], 2), vec![(3, 0.95), (1, 0.9)]);
    }

    #[test]
    fn tie_breaking() {
        assert_eq!(top_k_ids(vec![(2, 0.9), (1, 0.9)], 2), vec![(1, 0.9), (2, 0.9)]);
    }
}