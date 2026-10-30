/// Spectral drift measures how much a vector changes between two operator
/// applications. It is defined as:
///
///     drift = ||z_next - z_prev||
///
/// Normalized by vector length to avoid scale explosion.
///
/// This is used to detect instability, explosion, or collapse tendencies.
pub fn spectral_drift(z_prev: &[f64], z_next: &[f64]) -> f64 {
    if z_prev.len() != z_next.len() {
        return 0.0;
    }

    let mut acc = 0.0;

    for i in 0..z_prev.len() {
        let diff = z_next[i] - z_prev[i];
        acc += diff * diff;
    }

    let norm = acc.sqrt();
    norm / (z_prev.len() as f64)
}
