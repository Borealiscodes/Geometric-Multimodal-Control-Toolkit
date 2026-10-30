use crate::safety::engine::SafetyEngine;
use crate::safety::tuning::AdaptiveEnvelope;

/// TunedSafetyEngine wraps SafetyEngine and applies adaptive blending.
pub struct TunedSafetyEngine {
    base: SafetyEngine,
    adaptive: AdaptiveEnvelope,
}

impl TunedSafetyEngine {
    pub fn new() -> Self {
        TunedSafetyEngine {
            base: SafetyEngine::new(),
            adaptive: AdaptiveEnvelope::new(),
        }
    }

    pub fn step(&self, z_prev: &[f64], z_next: &[f64]) -> Option<Vec<f64>> {
        let raw = self.base.step(z_prev, z_next)?;

        // Apply soft limits for curvature and drift
        let soft = self.adaptive.soft_limit(&raw, crate::CURVATURE_BOUND);

        // Blend raw and soft corrections
        let blended = self.adaptive.adapt(&raw, &soft);

        Some(blended)
    }
}
