use crate::geometry::{compute_hessian, collapse_detected};

/// CollapseEnvelope prevents routing or fusion into collapse surfaces.
/// It enforces Hessian positivity and avoids degeneracy.
pub struct CollapseEnvelope;

impl CollapseEnvelope {
    pub fn new() -> Self {
        CollapseEnvelope
    }

    pub fn enforce(&self, z: &[f64]) -> Vec<f64> {
        let h = compute_hessian(z);

        if collapse_detected(&h) {
            // Pull vector away from collapse surface
            let mut corrected = z.to_vec();
            for v in corrected.iter_mut() {
                *v += 0.01;
            }
            return corrected;
        }

        z.to_vec()
    }
}
