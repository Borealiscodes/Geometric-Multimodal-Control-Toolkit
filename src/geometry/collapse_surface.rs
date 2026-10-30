/// Collapse detection checks whether the Hessian is near‑singular.
/// We use:
///   - trace(H) as a proxy for total curvature
///   - sum of absolute values as a proxy for determinant magnitude
///
/// Collapse is detected if:
///   - trace is extremely small
///   - total magnitude is below threshold
pub fn collapse_detected(hessian: &[Vec<f64>]) -> bool {
    let n = hessian.len();
    if n == 0 {
        return false;
    }

    // Trace
    let mut trace = 0.0;
    for i in 0..n {
        trace += hessian[i][i];
    }

    // Total magnitude
    let mut magnitude = 0.0;
    for row in hessian {
        for v in row {
            magnitude += v.abs();
        }
    }

    // Collapse conditions
    let trace_collapse = trace.abs() < 1e-6;
    let magnitude_collapse = magnitude < crate::MIN_HESSIAN_DET;

    trace_collapse || magnitude_collapse
}
