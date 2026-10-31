use serde::{Serialize, Deserialize};
use crate::geometry::collapse_surface::CollapseSurface;
use crate::geometry::collapse_curvature::CurvatureTensor;
use crate::io::{load_surface, save_curvature};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Batch22Output {
    pub curvature: CurvatureTensor,
    pub instability_zones: Vec<usize>,
}

pub fn run_batch22(surface_path: &str, output_path: &str, threshold: f64) -> Batch22Output {
    let surface: CollapseSurface = load_surface(surface_path);

    let curvature = CurvatureTensor::compute(&surface);
    let instability_zones = curvature.instability_zones(threshold);

    save_curvature(output_path, &curvature);

    Batch22Output {
        curvature,
        instability_zones,
    }
}
