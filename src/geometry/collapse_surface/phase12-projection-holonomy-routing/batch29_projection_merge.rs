use crate::geometry::collapse_surface::phase12_projection_holonomy_routing::projection_operator::HolonomyProjectionOperator;
use crate::geometry::collapse_surface::phase12_projection_holonomy_routing::merge_operator::HolonomyMergeOperator;

pub fn batch29_projection_merge<T: Copy>(
    transported: &Vec<T>,
    paths: &Vec<Vec<usize>>,
    manifold_size: usize
) -> Vec<Option<T>> {

    // Step 1: project transported quantities into manifold regions
    let projected = HolonomyProjectionOperator::project(transported, paths, manifold_size);

    // Step 2: merge overlapping projections
    HolonomyMergeOperator::merge(&projected)
}
