use serde::{Serialize, Deserialize};
use crate::geometry::collapse_surface::CollapseSurface;
use crate::geometry::collapse_curvature::CurvatureTensor;
use crate::geometry::collapse_alignment::AlignmentTensor;
use crate::io::{load_surface, load_curvature, save_alignment};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Batch23Output {
    pub alignment: AlignmentTensor,
}

pub fn run_batch23(surface_path: &str, curvature_path: &str, output_path: &str) -> Batch23Output {
    let surface: CollapseSurface = load_surface(surface_path);
    let curvature: CurvatureTensor = load_curvature(curvature_path);

    let alignment = AlignmentTensor::align(&surface, &curvature);

    save_alignment(output_path, &alignment);

    Batch23Output { alignment }
}
