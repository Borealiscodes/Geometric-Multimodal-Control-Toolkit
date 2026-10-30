use crate::operators::FusionOperator;
use crate::geometry::curvature;

/// FusionEnvelope wraps FusionOperator and enforces curvature bounds.
pub struct FusionEnvelope {
    fusion: FusionOperator,
}

impl FusionEnvelope {
    pub fn new() -> Self {
        FusionEnvelope {
            fusion: FusionOperator::new(),
        }
    }

    pub fn apply(&self, z1: &[f64], z2: &[f64]) -> Vec<f64> {
        let fused = self.fusion.apply(z1, z2);

        let curv = curvature(&fused);
        if curv.abs() > crate::CURVATURE_BOUND {
            let scale = crate::CURVATURE_BOUND / curv.abs();
            return fused.iter().map(|v| v * scale).collect();
        }

        fused
    }
}
