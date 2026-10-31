use crate::geometry::projection_seeds::projection_seed::ProjectionSeed;

/// SeedGeometryEnvelope unifies curvature, alignment, holonomy response,
/// and collapse proximity into a single stable geometry envelope.
/// This is the final operator of Phase 9.
#[derive(Clone, Debug)]
pub struct SeedGeometryEnvelope {
    pub curvature: f64,
    pub alignment: f64,
    pub holonomy: f64,
    pub collapse_proximity: f64,
    pub stability: f64,
}

impl SeedGeometryEnvelope {
    pub fn from_seed(seed: &ProjectionSeed) -> Self {
        let curvature = seed.curvature;
        let alignment = seed.alignment_bias;
        let holonomy = seed.holonomy_bias;
        let collapse_proximity = seed.collapse_proximity;

        // Stability is a weighted combination of all geometry signals.
        // Pre-Phase-10 stability envelope.
        let stability = Self::compute_stability(
            curvature,
            alignment,
            holonomy,
            collapse_proximity,
        );

        SeedGeometryEnvelope {
            curvature,
            alignment,
            holonomy,
            collapse_proximity,
            stability,
        }
    }

    /// Stability function for Phase 9 geometry.
    /// Phase 10 will replace this with collapse-surface geometry.
    fn compute_stability(
        curvature: f64,
        alignment: f64,
        holonomy: f64,
        collapse_proximity: f64,
    ) -> f64 {
        let base = curvature.abs() * 0.4
            + alignment.abs() * 0.3
            + holonomy.abs() * 0.2;

        // Collapse proximity reduces stability.
        let collapse_factor = 1.0 - (collapse_proximity * 0.5);

        let stability = base * collapse_factor;

        // Clamp stability to safe range.
        if stability > 10.0 {
            10.0
        } else if stability < 0.0 {
            0.0
        } else {
            stability
        }
    }
}
