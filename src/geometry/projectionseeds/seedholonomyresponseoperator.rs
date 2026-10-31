use crate::geometry::projection_seeds::projection_seed::ProjectionSeed;
use crate::runtime::holonomy_runtime::HolonomyRuntime;
use crate::ecology::ecology_runtime::EcologyRuntime;

/// SeedHolonomyResponseOperator applies holonomy-driven transforms to seeds.
/// Holonomy influences curvature, alignment, and gradient behavior.
pub struct SeedHolonomyResponseOperator;

impl SeedHolonomyResponseOperator {
    pub fn new() -> Self {
        SeedHolonomyResponseOperator
    }

    /// Apply holonomy response to curvature and alignment.
    pub fn apply_holonomy_response(
        &self,
        seed: &mut ProjectionSeed,
        holonomy: &HolonomyRuntime,
        ecology: &EcologyRuntime,
    ) {
        let hol = holonomy.holonomy_strength();
        let eco_align = ecology.alignment_field.strength();

        // Holonomy-driven curvature transform
        seed.curvature += hol * 0.2;

        // Holonomy-driven alignment transform
        seed.alignment_bias += hol * 0.1;

        // Holonomy gradient modulation
        let gradient_factor = 1.0 + (hol * eco_align * 0.02);
        seed.curvature *= gradient_factor;

        // Stability clamps (pre-envelope)
        if seed.curvature > 12.0 {
            seed.curvature = 12.0;
        }
        if seed.curvature < -12.0 {
            seed.curvature = -12.0;
        }

        if seed.alignment_bias > 6.0 {
            seed.alignment_bias = 6.0;
        }
        if seed.alignment_bias < -6.0 {
            seed.alignment_bias = -6.0;
        }

        // Record holonomy bias
        seed.holonomy_bias = hol;
    }

    /// Batch holonomy response
    pub fn apply_batch(
        &self,
        seeds: &mut [ProjectionSeed],
        holonomy: &HolonomyRuntime,
        ecology: &EcologyRuntime,
    ) {
        for seed in seeds.iter_mut() {
            self.apply_holonomy_response(seed, holonomy, ecology);
        }
    }
}
