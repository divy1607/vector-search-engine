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
