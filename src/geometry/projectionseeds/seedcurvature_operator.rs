use crate::geometry::projection_seeds::projection_seed::ProjectionSeed;
use crate::runtime::holonomy_runtime::HolonomyRuntime;
use crate::ecology::ecology_runtime::EcologyRuntime;

/// SeedCurvatureOperator applies curvature transforms to projection-seeds.
/// Curvature is influenced by ecology curvature, holonomy bias, and alignment.
pub struct SeedCurvatureOperator;

impl SeedCurvatureOperator {
    pub fn new() -> Self {
        SeedCurvatureOperator
    }

    /// Apply curvature transform using ecology curvature and holonomy bias.
    pub fn apply_curvature(
        &self,
        seed: &mut ProjectionSeed,
        holonomy: &HolonomyRuntime,
        ecology: &EcologyRuntime,
    ) {
        let hol = holonomy.holonomy_strength();
        let eco_curv = ecology.curvature_field.strength();
        let eco_align = ecology.alignment_field.strength();

        // Base curvature from ecology
        seed.curvature = eco_curv;

        // Holonomy-curvature coupling
        seed.curvature += hol * 0.15;

        // Alignment-curvature modulation
        seed.curvature *= 1.0 + (eco_align * 0.05);

        // Stability clamp (pre-envelope)
        if seed.curvature > 10.0 {
            seed.curvature = 10.0;
        }
        if seed.curvature < -10.0 {
            seed.curvature = -10.0;
        }
    }

    /// Batch curvature transform
    pub fn apply_batch(
        &self,
        seeds: &mut [ProjectionSeed],
        holonomy: &HolonomyRuntime,
        ecology: &EcologyRuntime,
    ) {
        for seed in seeds.iter_mut() {
            self.apply_curvature(seed, holonomy, ecology);
        }
    }
}
