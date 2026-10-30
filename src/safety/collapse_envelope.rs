use crate::geometry::{compute_hessian, collapse_detected};
use crate::safety::tuning::smooth_damp;

pub struct CollapseEnvelope;

impl CollapseEnvelope {
    pub fn new() -> Self { CollapseEnvelope }

    pub fn enforce(&self, z: &[f64]) -> Vec<f64> {
        let h = compute_hessian(z);

        if collapse_detected(&h) {
            return z.iter().map(|v| v + smooth_damp(0.01)).collect();
        }

        z.to_vec()
    }
}
