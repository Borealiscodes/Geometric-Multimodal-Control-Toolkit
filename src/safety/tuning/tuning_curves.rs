/// Smooth damping curve:
///     damp(x) = x / (1 + x)
/// This prevents runaway corrections and envelope oscillation.
pub fn smooth_damp(x: f64) -> f64 {
    x / (1.0 + x.abs())
}

/// Soft threshold curve:
///     soft(x, T) = x * exp(-(x/T)^2)
/// This gently suppresses values near threshold without hard clamping.
pub fn soft_threshold(x: f64, threshold: f64) -> f64 {
    let ratio = x / threshold;
    x * (-ratio * ratio).exp()
}

/// Envelope blending:
///     blend(a, b, w) = (1-w)*a + w*b
/// w ∈ [0,1]
pub fn blend(a: &[f64], b: &[f64], w: f64) -> Vec<f64> {
    a.iter().zip(b.iter()).map(|(x, y)| (1.0 - w) * x + w * y).collect()
}
