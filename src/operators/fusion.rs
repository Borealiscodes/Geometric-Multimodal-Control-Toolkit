use crate::geometry::curvature;

/// FusionOperator performs curvature-aware interpolation between two points
/// in latent or geometric space. The fusion is weighted and normalized to
/// avoid curvature spikes and drift.
pub struct FusionOperator {
    pub alpha: f64, // fusion weight
}

impl FusionOperator {
    pub fn new() -> Self {
        FusionOperator { alpha: 0.5 }
    }

    /// Curvature-aware fusion:
    ///   z_fused = (1 - α) * z1 + α * z2
    /// Then curvature is checked and normalized if needed.
    pub fn apply(&self, z1: &[f64], z2: &[f64]) -> Vec<f64> {
        let mut fused = Vec::with_capacity(z1.len());

        for i in 0..z1.len() {
            let v = (1.0 - self.alpha) * z1[i] + self.alpha * z2[i];
            fused.push(v);
        }

        // Curvature normalization step
        let curv = curvature(&fused);
        if curv.abs() > crate::CURVATURE_BOUND {
            let scale = crate::CURVATURE_BOUND / curv.abs();
            for i in 0..fused.len() {
                fused[i] *= scale;
            }
        }

        fused
    }
}
