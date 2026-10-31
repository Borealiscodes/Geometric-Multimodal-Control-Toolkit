use serde::{Serialize, Deserialize};
use crate::geometry::collapse_alignment::AlignmentTensor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityEnvelope {
    pub compatible: Vec<bool>,
}

impl CompatibilityEnvelope {
    pub fn evaluate(alignment: &AlignmentTensor, matrix: &[f64]) -> Self {
        let mut compatible = Vec::new();

        for (i, vec) in alignment.vectors.iter().enumerate() {
            let m = matrix[i];
            let score = vec.x.abs() + vec.y.abs() + vec.z.abs();
            compatible.push(score <= m);
        }

        CompatibilityEnvelope { compatible }
    }

    pub fn incompatible_regions(&self) -> Vec<usize> {
        let mut regions = Vec::new();
        for (i, ok) in self.compatible.iter().enumerate() {
            if !ok {
                regions.push(i);
            }
        }
        regions
    }
}
