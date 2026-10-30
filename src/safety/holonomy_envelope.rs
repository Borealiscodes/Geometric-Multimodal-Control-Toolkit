use crate::runtime::holonomy_runtime::HolonomyRuntime;

/// HolonomyEnvelope prevents holonomy runaway and enforces stability.
pub struct HolonomyEnvelope {
    pub max_strength: f64,
}

impl HolonomyEnvelope {
    pub fn new(max_strength: f64) -> Self {
        HolonomyEnvelope { max_strength }
    }

    pub fn enforce(&self, runtime: &HolonomyRuntime) -> bool {
        runtime.holonomy_strength().abs() <= self.max_strength
    }
}
