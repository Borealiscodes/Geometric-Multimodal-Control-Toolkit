/// Hessian approximation using outer products of gradient differences.
/// This is not a full symbolic Hessian — it is a runtime stability Hessian
/// designed to detect degeneracy and collapse tendencies.
///
/// For z, we compute:
///     H[i][j] = (z[i] - z[i-1]) * (z[j] - z[j-1])
///
/// This produces a positive semi‑definite matrix suitable for spectral checks.
pub fn compute_hessian(z: &[f64]) -> Vec<Vec<f64>> {
    let n = z.len();
    if n < 2 {
        return vec![vec![0.0]];
    }

    // Compute gradient
    let mut grad = Vec::with_capacity(n);
    grad.push(0.0); // first gradient undefined → treat as zero
    for i in 1..n {
        grad.push(z[i] - z[i - 1]);
    }

    // Build Hessian as outer product of gradient
    let mut hessian = vec![vec![0.0; n]; n];

    for i in 0..n {
        for j in 0..n {
            hessian[i][j] = grad[i] * grad[j];
        }
    }

    hessian
}
