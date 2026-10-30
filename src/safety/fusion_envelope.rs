use crate::operators::FusionOperator;
use crate::geometry::curvature;
use crate::safety::tuning::soft_threshold;

pub struct FusionEnvelope {
    fusion: FusionOperator,
}

impl FusionEnvelope {
    pub fn new() -> Self {
        FusionEnvelope { fusion: FusionOperator::new() }
    }

    pub fn apply(&self, z1: &[f64], z2: &[f64]) -> Vec<f64> {
        let fused = self.fusion.apply(z1, z2);
        let curv = curvature(&fused);

        if curv.abs() > crate::CURVATURE_BOUND {
            return fused.iter().map(|v| soft_threshold(*v, crate::CURVATURE_BOUND)).collect();
        }

        fused
    }
}
