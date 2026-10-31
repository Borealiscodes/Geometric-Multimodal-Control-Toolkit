use crate::worlds::SoftWorld;
use crate::worlds::world_rules::WorldRule;

/// WorldRuleRuntime applies world-rules to SoftWorld.
/// This is the first rule-bearing world-geometry layer.
pub struct WorldRuleRuntime {
    pub world: SoftWorld,
}

impl WorldRuleRuntime {
    pub fn new() -> Self {
        WorldRuleRuntime {
            world: SoftWorld::new(),
        }
    }

    /// Apply world-rule by adjusting world geometry parameters.
    pub fn apply_rule(&mut self, rule: &WorldRule) {
        // Adjust world holonomy
        self.world.holonomy_scale *= 1.0 + rule.holonomy_bias * rule.weight;

        // Adjust world alignment
        self.world.alignment_scale *= 1.0 + rule.alignment_bias * rule.weight;

        // Adjust world curvature
        self.world.curvature_scale *= 1.0 + rule.curvature_bias * rule.weight;
    }
}
