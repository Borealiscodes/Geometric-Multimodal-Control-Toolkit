use crate::ecology::ecology_runtime::EcologyRuntime;
use crate::runtime::holonomy_runtime::HolonomyRuntime;
use crate::worlds::world_rules::WorldRule;

/// WorldRuleSynthesizer generates world-rules from ecology + holonomy.
/// This is the core of Batch 13.
pub struct WorldRuleSynthesizer;

impl WorldRuleSynthesizer {
    pub fn new() -> Self {
        WorldRuleSynthesizer
    }

    pub fn synthesize(
        &self,
        ecology: &EcologyRuntime,
        holonomy: &HolonomyRuntime,
    ) -> WorldRule {
        let curv = ecology.curvature_field.strength();
        let align = ecology.alignment_field.strength();
        let hol = holonomy.holonomy_strength();

        // Rule weight is proportional to ecology + holonomy magnitude
        let weight = (curv.abs() + align.abs() + hol.abs()).min(1.0);

        WorldRule::new(weight, curv, align, hol)
    }
}
