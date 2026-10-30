/// EcologyCurvature accumulates curvature over time.
/// This is distinct from operator curvature (Phase 6).
pub struct EcologyCurvature {
    pub value: f64,
    pub decay_rate: f64,
}

impl EcologyCurvature {
    pub fn new(decay_rate: f64) -> Self {
        EcologyCurvature {
            value: 0.0,
            decay_rate,
        }
    }

    /// Accumulate curvature from world geometry.
    pub fn accumulate(&mut self, curv: f64) {
        self.value += curv;
    }

    /// Apply decay to prevent runaway curvature ecology.
    pub fn decay(&mut self) {
        self.value *= (1.0 - self.decay_rate).max(0.0);
    }

    pub fn strength(&self) -> f64 {
        self.value
    }
}
