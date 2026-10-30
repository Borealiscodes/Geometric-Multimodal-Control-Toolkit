/// alignment_score measures multimodal alignment between two vectors.
/// It is defined as:
///
///     score = Σ (z1[i] * z2[i])
///
/// Normalized by vector length to avoid scale explosion.
///
/// This is a simple coherence signal used by RoutingOperator and SoftWorld.
/// Phase 7 will replace this with a full multimodal alignment operator.
pub fn alignment_score(z1: &[f64], z2: &[f64]) -> f64 {
    if z1.len() != z2.len() {
        return 0.0;
    }

    let mut acc = 0.0;

    for i in 0..z1.len() {
        acc += z1[i] * z2[i];
    }

    acc / (z1.len() as f64)
}
