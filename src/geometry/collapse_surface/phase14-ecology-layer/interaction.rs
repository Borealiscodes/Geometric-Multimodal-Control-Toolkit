/// Phase 14D — Interaction Logic Implementation
/// Applies ecological coupling between adjacent regions.
/// This is the first propagation layer for routed fields.

use crate::geometry::collapse_surface::phase13_global_invariants::{
    RoutedNormal, RoutedCurvature, RoutedCompatibility,
};

use crate::geometry::collapse_surface::phase14_ecology_layer::operators::InteractionResult;

/// Placeholder ecology coefficient (Phase 15 will refine)
const ECOLOGY_COEFF: f32 = 0.25;

/// Linear interpolation helper
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Phase 14D interaction implementation
pub struct InteractionLogic;

impl InteractionLogic {
    pub fn normals(
        normals: &Vec<RoutedNormal>,
        adjacency: &Vec<(usize, usize)>
    ) -> InteractionResult<RoutedNormal> {
        let mut adjusted = normals.clone();

        for (i, j) in adjacency {
            let a = &normals[*i];
            let b = &normals[*j];

            let ai = RoutedNormal {
                x: lerp(a.x, b.x, ECOLOGY_COEFF),
                y: lerp(a.y, b.y, ECOLOGY_COEFF),
                z: lerp(a.z, b.z, ECOLOGY_COEFF),
            };

            let bj = RoutedNormal {
                x: lerp(b.x, a.x, ECOLOGY_COEFF),
                y: lerp(b.y, a.y, ECOLOGY_COEFF),
                z: lerp(b.z, a.z, ECOLOGY_COEFF),
            };

            adjusted[*i] = ai;
            adjusted[*j] = bj;
        }

        InteractionResult { adjusted }
    }

    pub fn curvature(
        curvature: &Vec<RoutedCurvature>,
        adjacency: &Vec<(usize, usize)>
    ) -> InteractionResult<RoutedCurvature> {
        let mut adjusted = curvature.clone();

        for (i, j) in adjacency {
            let a = curvature[*i].k;
            let b = curvature[*j].k;

            let ai = RoutedCurvature { k: lerp(a, b, ECOLOGY_COEFF) };
            let bj = RoutedCurvature { k: lerp(b, a, ECOLOGY_COEFF) };

            adjusted[*i] = ai;
            adjusted[*j] = bj;
        }

        InteractionResult { adjusted }
    }

    pub fn compatibility(
        compat: &Vec<RoutedCompatibility>,
        adjacency: &Vec<(usize, usize)>
    ) -> InteractionResult<RoutedCompatibility> {
        let mut adjusted = compat.clone();

        for (i, j) in adjacency {
            let a = compat[*i].c;
            let b = compat[*j].c;

            let ai = RoutedCompatibility { c: lerp(a, b, ECOLOGY_COEFF) };
            let bj = RoutedCompatibility { c: lerp(b, a, ECOLOGY_COEFF) };

            adjusted[*i] = ai;
            adjusted[*j] = bj;
        }

        InteractionResult { adjusted }
    }
}
