use crate::geometry::projection_seeds::projection_seed::ProjectionSeed;
use crate::runtime::holonomy_runtime::HolonomyRuntime;
use crate::ecology::ecology_runtime::EcologyRuntime;
use crate::worlds::world_rule_runtime::WorldRuleRuntime;

/// ProjectionSeedGenerator creates seeds from Phase 8 signals.
/// This is the first geometry-bearing operator of Phase 9.
pub struct ProjectionSeedGenerator;

impl ProjectionSeedGenerator {
    pub fn new() -> Self {
        ProjectionSeedGenerator
    }

    /// Generate a single projection-seed from holonomy, ecology, and world-rules.
    pub fn generate(
        &self,
        holonomy: &HolonomyRuntime,
        ecology: &EcologyRuntime,
        rules: &WorldRuleRuntime,
    ) -> ProjectionSeed {
        let hol = holonomy.holonomy_strength();
        let curv = ecology.curvature_field.strength();
        let align = ecology.alignment_field.strength();
        let collapse = rules.collapse_proximity();

        let mut seed = ProjectionSeed::new(curv, hol, align, collapse);

        seed.apply_holonomy(hol);
        seed.apply_alignment(align);
        seed.set_collapse_proximity(collapse);

        seed
    }

    /// Generate multiple seeds from two ecology vectors.
    pub fn generate_batch(
        &self,
        holonomy: &HolonomyRuntime,
        ecology: &EcologyRuntime,
        rules: &WorldRuleRuntime,
        count: usize,
    ) -> Vec<ProjectionSeed> {
        (0..count)
            .map(|_| self.generate(holonomy, ecology, rules))
            .collect()
    }
      }
