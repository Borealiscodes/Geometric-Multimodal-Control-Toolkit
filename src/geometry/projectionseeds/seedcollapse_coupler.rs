use crate::geometry::projection_seeds::projection_seed::ProjectionSeed;
use crate::worlds::world_rule_runtime::WorldRuleRuntime;

/// SeedCollapseCoupler applies collapse-surface proximity transforms to seeds.
/// Collapse coupling is pre-Phase 10: proximity-aware, not collapse-active.
pub struct SeedCollapseCoupler;

impl SeedCollapseCoupler {
    pub fn new() -> Self {
        SeedCollapseCoupler
    }

    /// Apply collapse proximity transform.
    pub fn apply_collapse_coupling(
        &self,
        seed: &mut ProjectionSeed,
        rules: &WorldRuleRuntime,
    ) {
        let collapse_prox = rules.collapse_proximity();

        // Record collapse proximity
        seed.collapse_proximity = collapse_prox;

        // Collapse-curvature modulation
        seed.curvature *= 1.0 - (collapse_prox * 0.05);

        // Collapse-alignment modulation
        seed.alignment_bias *= 1.0 - (collapse_prox * 0.03);

        // Collapse safety clamp (pre-Phase 10)
        if seed.collapse_proximity > 1.0 {
            seed.collapse_proximity = 1.0;
        }
        if seed.collapse_proximity < 0.0 {
            seed.collapse_proximity = 0.0;
        }

        // Curvature safety clamp
        if seed.curvature > 12.0 {
            seed.curvature = 12.0;
        }
        if seed.curvature < -12.0 {
            seed.curvature = -12.0;
        }

        // Alignment safety clamp
        if seed.alignment_bias > 6.0 {
            seed.alignment_bias = 6.0;
        }
        if seed.alignment_bias < -6.0 {
            seed.alignment_bias = -6.0;
        }
    }

    /// Batch collapse coupling
    pub fn apply_batch(
        &self,
        seeds: &mut [ProjectionSeed],
        rules: &WorldRuleRuntime,
    ) {
        for seed in seeds.iter_mut() {
            self.apply_collapse_coupling(seed, rules);
        }
    }
}
