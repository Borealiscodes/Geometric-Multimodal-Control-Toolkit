use crate::geometry::collapse_surface::holonomy_transport::HolonomyTransportOperator;

pub fn batch25_holonomy_normals(
    normals: &Vec<[f32; 3]>,
    paths: &Vec<Vec<usize>>
) -> Vec<[f32; 3]> {

    // Orchestrates holonomy transport of normals
    HolonomyTransportOperator::transport_normals(normals, paths)
}
