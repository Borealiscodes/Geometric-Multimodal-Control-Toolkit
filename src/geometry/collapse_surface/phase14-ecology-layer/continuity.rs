/// Phase 14B — Continuity Logic Implementation
/// Implements continuity metrics for normals, curvature, and compatibility.
/// This is the first computational layer enforcing Phase 13 invariants.

use crate::geometry::collapse_surface::phase13_global_invariants::{
    RoutedNormal, RoutedCurvature, RoutedCompatibility,
};

use crate::geometry::collapse_surface::phase14_ecology_layer::operators::ContinuityMetric;

/// Compute L2 distance between two normals
fn continuity_normals(a: &RoutedNormal, b: &RoutedNormal) -> f32 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

/// Compute absolute curvature difference
fn continuity_curvature(a: &RoutedCurvature, b: &RoutedCurvature) -> f32 {
    (a.k - b.k).abs()
}

/// Compute absolute compatibility difference
fn continuity_compatibility(a: &RoutedCompatibility, b: &RoutedCompatibility) -> f32 {
    (a.c - b.c).abs()
}

/// Phase 14B continuity implementation
pub struct ContinuityLogic;

impl ContinuityLogic {
    pub fn normals(
        normals: &Vec<RoutedNormal>,
        adjacency: &Vec<(usize, usize)>
    ) -> Vec<ContinuityMetric> {
        adjacency.iter().map(|(i, j)| {
            let v = continuity_normals(&normals[*i], &normals[*j]);
            ContinuityMetric { value: v }
        }).collect()
    }

    pub fn curvature(
        curvature: &Vec<RoutedCurvature>,
        adjacency: &Vec<(usize, usize)>
    ) -> Vec<ContinuityMetric> {
        adjacency.iter().map(|(i, j)| {
            let v = continuity_curvature(&curvature[*i], &curvature[*j]);
            ContinuityMetric { value: v }
        }).collect()
    }

    pub fn compatibility(
        compat: &Vec<RoutedCompatibility>,
        adjacency: &Vec<(usize, usize)>
    ) -> Vec<ContinuityMetric> {
        adjacency.iter().map(|(i, j)| {
            let v = continuity_compatibility(&compat[*i], &compat[*j]);
            ContinuityMetric { value: v }
        }).collect()
    }
}
