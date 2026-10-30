/// HolonomyField tracks holonomy accumulation over time.
/// It is the core ecological signal for Phase 8.
pub struct HolonomyField {
    pub value: f64,
    pub decay_rate: f64,
}

impl HolonomyField {
    pub fn new(decay_rate: f64) -> Self {
        HolonomyField {
            value: 0.0,
            decay_rate,
        }
    }

    /// Accumulate holonomy from a loop measurement.
    pub fn accumulate(&mut self, loop_value: f64) {
        self.value += loop_value;
    }

    /// Apply exponential decay to prevent runaway holonomy.
    pub fn decay(&mut self) {
        self.value *= (1.0 - self.decay_rate).max(0.0);
    }

    /// Return current holonomy field strength.
    pub fn strength(&self) -> f64 {
        self.value
    }
}
