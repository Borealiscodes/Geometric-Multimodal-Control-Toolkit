/// Phase 14C — Stability Logic Implementation
/// Applies ecological smoothing based on continuity metrics.
/// This is the first enforcement layer for Phase 13 invariants.

use crate::geometry::collapse_surface::phase13_global_invariants::{
    RoutedNormal, RoutedCurvature, RoutedCompatibility,
};

use crate::geometry::collapse_surface::phase14_ecology_layer::operators::{
    ContinuityMetric, StabilityResult,
};

/// Stability threshold placeholder (Phase 15 will refine)
const STABILITY_THRESHOLD: f32 = 0.1;

/// Phase 14C stability implementation
pub struct StabilityLogic;

impl StabilityLogic {
    pub fn normals(
        continuity: &Vec<ContinuityMetric>
    ) -> StabilityResult {
        let mut max_instability = 0.0;
        for metric in continuity {
            if metric.value > STABILITY_THRESHOLD {
                let diff = metric.value - STABILITY_THRESHOLD;
                if diff > max_instability {
                    max_instability = diff;
                }
            }
        }

        if max_instability > 0.0 {
            StabilityResult::Unstable(max_instability)
        } else {
            StabilityResult::Stable
        }
    }

    pub fn curvature(
        continuity: &Vec<ContinuityMetric>
    ) -> StabilityResult {
        let mut max_instability = 0.0;
        for metric in continuity {
            if metric.value > STABILITY_THRESHOLD {
                let diff = metric.value - STABILITY_THRESHOLD;
                if diff > max_instability {
                    max_instability = diff;
                }
            }
        }

        if max_instability > 0.0 {
            StabilityResult::Unstable(max_instability)
        } else {
            StabilityResult::Stable
        }
    }

    pub fn compatibility(
        continuity: &Vec<ContinuityMetric>
    ) -> StabilityResult {
        let mut max_instability = 0.0;
        for metric in continuity {
            if metric.value > STABILITY_THRESHOLD {
                let diff = metric.value - STABILITY_THRESHOLD;
                if diff > max_instability {
                    max_instability = diff;
                }
            }
        }

        if max_instability > 0.0 {
            StabilityResult::Unstable(max_instability)
        } else {
            StabilityResult::Stable
        }
    }
}
