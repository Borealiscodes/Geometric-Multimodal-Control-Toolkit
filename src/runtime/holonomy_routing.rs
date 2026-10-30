use crate::runtime::holonomy_runtime::HolonomyRuntime;

/// HolonomyRouting adjusts routing based on holonomy field strength.
/// This is the first holonomy-aware operator in the system.
pub struct HolonomyRouting {
    pub sensitivity: f64,
}

impl HolonomyRouting {
    pub fn new(sensitivity: f64) -> Self {
        HolonomyRouting { sensitivity }
    }

    /// Route vector based on holonomy field strength.
    pub fn route(&self, z: &[f64], runtime: &HolonomyRuntime) -> Vec<f64> {
        let hol = runtime.holonomy_strength();
        let factor = 1.0 + self.sensitivity * hol;

        z.iter().map(|v| v * factor).collect()
    }
}
