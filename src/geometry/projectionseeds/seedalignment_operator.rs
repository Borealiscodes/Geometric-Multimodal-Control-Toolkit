use crate::geometry::projection_seeds::projection_seed::ProjectionSeed;
use crate::runtime::holonomy_runtime::HolonomyRuntime;
use crate::ecology::ecology_runtime::EcologyRuntime;

/// SeedAlignmentOperator applies alignment transforms to projection-seeds.
/// Alignment is influenced by ecology alignment and holonomy-alignment coupling.
pub struct SeedAlignmentOperator;

impl SeedAlignmentOperator {
    pub fn new() -> Self {
        SeedAlignmentOperator
    }

    /// Apply alignment transform using ecology alignment and holonomy bias.
    pub fn apply_alignment(
        &self,
        seed: &mut ProjectionSeed,
        holonomy: &HolonomyRuntime,
        ecology: &EcologyRuntime,
    ) {
        let hol = holonomy.holonomy_strength();
        let eco_align = ecology.alignment_field.strength();

        // Base alignment from ecology
        seed.alignment_bias = eco_align;

        // Holonomy-alignment coupling
        seed.alignment_bias += hol * 0.12;

        // Alignment-curvature modulation
        seed.curvature *= 1.0 + (seed.alignment_bias * 0.03);

        // Stability clamp (pre-envelope)
        if seed.alignment_bias > 5.0 {
            seed.alignment_bias = 5.0;
        }
        if seed.alignment_bias < -5.0 {
            seed.alignment_bias = -5.0;
        }
    }

    /// Batch alignment transform
    pub fn apply_batch(
        &self,
        seeds: &mut [ProjectionSeed],
        holonomy: &HolonomyRuntime,
        ecology: &EcologyRuntime,
    ) {
        for seed in seeds.iter_mut() {
            self.apply_alignment(seed, holonomy, ecology);
        }
    }
}
