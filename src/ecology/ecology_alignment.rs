/// EcologyAlignment tracks long-range multimodal alignment.
/// This is distinct from operator-level alignment (Phase 6).
pub struct EcologyAlignment {
    pub value: f64,
    pub decay_rate: f64,
}

impl EcologyAlignment {
    pub fn new(decay_rate: f64) -> Self {
        EcologyAlignment {
            value: 0.0,
            decay_rate,
        }
    }

    /// Accumulate alignment from world alignment score.
    pub fn accumulate(&mut self, align: f64) {
        self.value += align;
    }

    /// Apply decay to prevent runaway alignment ecology.
    pub fn decay(&mut self) {
        self.value *= (1.0 - self.decay_rate).max(0.0);
    }

    pub fn strength(&self) -> f64 {
        self.value
    }
}
