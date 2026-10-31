use crate::geometry::collapse_surface::phase12_projection_holonomy_routing::path_selector::HolonomyPathSelector;

pub fn batch28_path_selection(
    manifold_topology: &Vec<Vec<usize>>,
    seed_points: &Vec<usize>
) -> Vec<Vec<usize>> {

    // Orchestrates holonomy path selection
    HolonomyPathSelector::select_paths(manifold_topology, seed_points)
}
