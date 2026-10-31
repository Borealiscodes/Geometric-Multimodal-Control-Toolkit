use crate::geometry::collapse_surface::holonomy_compat::HolonomyCompatibilityOperator;

pub fn batch27_holonomy_alignment(
    compat: &Vec<f32>,
    paths: &Vec<Vec<usize>>
) -> Vec<f32> {

    // Orchestrates holonomy transport of compatibility envelope
    HolonomyCompatibilityOperator::transport_compatibility(compat, paths)
}
