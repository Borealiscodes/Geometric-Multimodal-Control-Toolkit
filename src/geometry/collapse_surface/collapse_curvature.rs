use serde::{Serialize, Deserialize};
use crate::geometry::collapse_surface::{CollapseSurface, Vector3};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurvatureTensor {
    pub values: Vec<f64>,
}

impl CurvatureTensor {
    pub fn compute(surface: &CollapseSurface) -> Self {
        let mut values = Vec::new();

        for normal in &surface.normals {
            let curvature = (normal.x.abs() + normal.y.abs() + normal.z.abs()) / 3.0;
            values.push(curvature);
        }

        CurvatureTensor { values }
    }

    pub fn instability_zones(&self, threshold: f64) -> Vec<usize> {
        let mut zones = Vec::new();
        for (i, v) in self.values.iter().enumerate() {
            if *v > threshold {
                zones.push(i);
            }
        }
        zones
    }
}
