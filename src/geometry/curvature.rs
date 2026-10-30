/// Curvature estimation using a simple second‑order finite difference.
/// This is a lightweight approximation suitable for runtime stability checks.
///
/// For a vector z = [z0, z1, ..., zn], curvature is approximated as:
///     κ = Σ |z[i+1] - 2*z[i] + z[i-1]|
/// Normalized by vector length to avoid scale explosion.
pub fn curvature(z: &[f64]) -> f64 {
    if z.len() < 3 {
        return 0.0;
    }

    let mut acc = 0.0;

    for i in 1..z.len() - 1 {
        let second_deriv = z[i + 1] - 2.0 * z[i] + z[i - 1];
        acc += second_deriv.abs();
    }

    acc / (z.len() as f64)
}
