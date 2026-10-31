use crate::worlds::world_rules::WorldRule;

/// WorldRuleEnvelope enforces coherence of synthesized rules.
/// Prevents contradictory or unstable rule sets.
pub struct WorldRuleEnvelope {
    pub max_bias: f64,
}

impl WorldRuleEnvelope {
    pub fn new(max_bias: f64) -> Self {
        WorldRuleEnvelope { max_bias }
    }

    pub fn enforce(&self, rule: &WorldRule) -> bool {
        rule.curvature_bias.abs() <= self.max_bias &&
        rule.alignment_bias.abs() <= self.max_bias &&
        rule.holonomy_bias.abs() <= self.max_bias
    }
}
