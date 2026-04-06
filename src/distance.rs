// distance.rs

pub fn squared_l2_distance(q: &[f32], v: &[f32]) -> f32 {
    assert_eq!(q.len(), v.len());
    let n = q.len();
    let mut l2: f32 = 0.0;
    for i in 0..n {
        let diff: f32 = q[i] - v[i];
        l2 += diff * diff;
    }
    return l2;
}

pub fn cosine_similarity(q: &[f32], v: &[f32]) -> f32 {
    assert_eq!(q.len(), v.len());
    let n = q.len();
    let mut num: f32 = 0.0;
    let mut den1: f32 = 0.0;
    let mut den2: f32 = 0.0;
    for i in 0..n {
        num += q[i] * v[i];
        den1 += q[i] * q[i];
        den2 += v[i] * v[i];
    }
    let den11 = den1.sqrt();
    let den22 = den2.sqrt();
    let den: f32 = den11 * den22;
    if den == 0.0 {
        return 0.0;
    }
    let cosine: f32 = num / den;
    return cosine;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_identical() {
        assert_eq!(cosine_similarity(&[1.0, 0.0], &[1.0, 0.0]), 1.0);
    }

    #[test]
    fn test_cosine_orthogonal() {
        assert_eq!(cosine_similarity(&[1.0, 0.0], &[0.0, 1.0]), 0.0);
    }

    #[test]
    fn test_cosine_opposite() {
        assert_eq!(cosine_similarity(&[1.0, 0.0], &[-1.0, 0.0]), -1.0);
    }

    #[test]
    fn test_cosine_zero() {
        assert_eq!(cosine_similarity(&[23.0, 56.2], &[0.0, 0.0]), 0.0);
    }

    #[test]
    fn test_l2() {
        assert_eq!(squared_l2_distance(&[0.0, 0.0], &[3.0, 4.0]), 25.0);
    }
}