use serde::{Serialize, Deserialize};
use crate::geometry::collapse_surface::{CollapseSurface, Vector3};
use crate::geometry::collapse_curvature::CurvatureTensor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlignmentTensor {
    pub vectors: Vec<Vector3>,
}

impl AlignmentTensor {
    pub fn align(surface: &CollapseSurface, curvature: &CurvatureTensor) -> Self {
        let mut vectors = Vec::new();

        for (i, normal) in surface.normals.iter().enumerate() {
            let c = curvature.values[i];
            let aligned = Vector3 {
                x: normal.x * (1.0 - c),
                y: normal.y * (1.0 - c),
                z: normal.z * (1.0 - c),
            };
            vectors.push(aligned.normalize());
        }

        AlignmentTensor { vectors }
    }
}
