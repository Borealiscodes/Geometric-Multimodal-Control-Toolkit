use crate::safety::tuning::{smooth_damp, soft_threshold, blend};

/// AdaptiveEnvelope applies tuning curves to envelope corrections.
/// It blends corrections instead of replacing them.
pub struct AdaptiveEnvelope;

impl AdaptiveEnvelope {
    pub fn new() -> Self {
        AdaptiveEnvelope
    }

    pub fn adapt(&self, original: &[f64], corrected: &[f64]) -> Vec<f64> {
        // Compute correction magnitude
        let mut mag = 0.0;
        for (o, c) in original.iter().zip(corrected.iter()) {
            mag += (c - o).abs();
        }

        // Smooth damping
        let w = smooth_damp(mag);

        // Blend original and corrected
        blend(original, corrected, w)
    }

    pub fn soft_limit(&self, z: &[f64], threshold: f64) -> Vec<f64> {
        z.iter().map(|v| soft_threshold(*v, threshold)).collect()
    }
}
