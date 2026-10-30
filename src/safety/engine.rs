use crate::safety::{
    spectral_envelope::SpectralEnvelope,
    collapse_envelope::CollapseEnvelope,
    fusion_envelope::FusionEnvelope,
    routing_envelope::RoutingEnvelope,
    world_envelope::WorldEnvelope,
};

/// SafetyEngine orchestrates all safety envelopes.
/// It is the top-level S-layer controller.
pub struct SafetyEngine {
    spectral: SpectralEnvelope,
    collapse: CollapseEnvelope,
    fusion: FusionEnvelope,
    routing: RoutingEnvelope,
    world: WorldEnvelope,
}

impl SafetyEngine {
    pub fn new() -> Self {
        SafetyEngine {
            spectral: SpectralEnvelope::new(),
            collapse: CollapseEnvelope::new(),
            fusion: FusionEnvelope::new(),
            routing: RoutingEnvelope::new(),
            world: WorldEnvelope::new(),
        }
    }

    pub fn step(&self, z_prev: &[f64], z_next: &[f64]) -> Option<Vec<f64>> {
        // Collapse envelope
        let safe = self.collapse.enforce(z_next);

        // Routing envelope
        let routed = self.routing.route(&safe);

        // Fusion envelope (self-fusion for stability)
        let fused = self.fusion.apply(&safe, &routed);

        // Spectral envelope
        let h = crate::geometry::compute_hessian(&fused);
        let spectral_safe = self.spectral.enforce(z_prev, &fused, &h);

        // World envelope
        if !self.world.enforce(&safe, &spectral_safe) {
            return None;
        }

        Some(spectral_safe)
    }
}
