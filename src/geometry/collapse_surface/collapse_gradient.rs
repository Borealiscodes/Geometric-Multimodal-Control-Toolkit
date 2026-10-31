use serde::{Serialize, Deserialize};
use crate::geometry::collapse_surface::{CollapseSurface, Vector3};
use crate::geometry::collapse_curvature::CurvatureTensor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradientField {
    pub gradients: Vec<Vector3>,
}

impl GradientField {
    pub fn generate(surface: &CollapseSurface, curvature: &CurvatureTensor) -> Self {
        let mut gradients = Vec::new();

        for (i, p) in surface.points.iter().enumerate() {
            let c = curvature.values[i];
            let g = Vector3 {
                x: p.x * c,
                y: p.y * c,
                z: p.z * c,
            };
            gradients.push(g.normalize());
        }

        GradientField { gradients }
    }
}
