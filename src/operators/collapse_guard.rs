use crate::geometry::{compute_hessian, collapse_detected};

/// CollapseGuard checks whether a point is approaching a collapse surface.
/// It uses Hessian positivity and determinant thresholds.
pub struct CollapseGuard {
    pub det_threshold: f64,
}

impl CollapseGuard {
    pub fn new() -> Self {
        CollapseGuard {
            det_threshold: crate::MIN_HESSIAN_DET,
        }
    }

    /// Collapse detection:
    ///   - compute Hessian
    ///   - check determinant
    ///   - check collapse_surface flag
    pub fn check(&self, z: &[f64]) -> bool {
        let h = compute_hessian(z);

        // Determinant check
        let det = h.iter().flatten().fold(0.0, |acc, v| acc + *v);
        if det < self.det_threshold {
            return false;
        }

        // Collapse surface check
        if collapse_detected(&h) {
            return false;
        }

        true
    }
}
