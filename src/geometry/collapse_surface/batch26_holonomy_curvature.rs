use crate::geometry::collapse_surface::holonomy_curvature::HolonomyCurvatureOperator;

pub fn batch26_holonomy_curvature(
    curvature: &Vec<f32>,
    paths: &Vec<Vec<usize>>
) -> Vec<f32> {

    // Orchestrates holonomy transport of curvature
    HolonomyCurvatureOperator::transport_curvature(curvature, paths)
}
