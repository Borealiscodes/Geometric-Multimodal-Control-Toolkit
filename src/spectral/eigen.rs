/// Eigenvalue extraction for positive semi-definite Hessians.
/// This uses a simple power-iteration approximation to compute the dominant
/// eigenvalue, then deflates and repeats for the remaining eigenvalues.
///
/// This is lightweight and stable enough for runtime stability checks.
pub fn eigenvalues(hessian: &[Vec<f64>]) -> Vec<f64> {
    let n = hessian.len();
    if n == 0 {
        return vec![0.0];
    }

    let mut eigenvals = Vec::new();
    let mut matrix = hessian.to_vec(); // local copy for deflation

    for _ in 0..n {
        // Power iteration
        let mut v = vec![1.0; n];
        for _ in 0..20 {
            let mut next = vec![0.0; n];
            for i in 0..n {
                for j in 0..n {
                    next[i] += matrix[i][j] * v[j];
                }
            }

            // Normalize
            let norm = next.iter().map(|x| x * x).sum::<f64>().sqrt();
            if norm > 0.0 {
                for i in 0..n {
                    v[i] = next[i] / norm;
                }
            }
        }

        // Rayleigh quotient for eigenvalue
        let mut lambda = 0.0;
        for i in 0..n {
            for j in 0..n {
                lambda += v[i] * matrix[i][j] * v[j];
            }
        }

        eigenvals.push(lambda.abs());

        // Deflation: subtract λ * v vᵀ
        for i in 0..n {
            for j in 0..n {
                matrix[i][j] -= lambda * v[i] * v[j];
            }
        }
    }

    eigenvals
}
