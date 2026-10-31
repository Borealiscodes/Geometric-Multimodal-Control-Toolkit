use crate::runtime::holonomy_runtime::HolonomyRuntime;
use crate::ecology::ecology_runtime::EcologyRuntime;
use crate::worlds::world_rules::WorldRule;

/// EcologySafetyEnvelope enforces stability across all Phase 8 signals:
/// - holonomy field
/// - ecology curvature field
/// - ecology alignment field
/// - world-rule coherence
pub struct EcologySafetyEnvelope {
    pub max_holonomy: f64,
    pub max_ecology_curvature: f64,
    pub max_ecology_alignment: f64,
    pub max_rule_bias: f64,
}

impl EcologySafetyEnvelope {
    pub fn new(
        max_holonomy: f64,
        max_ecology_curvature: f64,
        max_ecology_alignment: f64,
        max_rule_bias: f64,
    ) -> Self {
        EcologySafetyEnvelope {
            max_holonomy,
            max_ecology_curvature,
            max_ecology_alignment,
            max_rule_bias,
        }
    }

    pub fn enforce(
        &self,
        holonomy: &HolonomyRuntime,
        ecology: &EcologyRuntime,
        rule: &WorldRule,
    ) -> bool {
        let hol = holonomy.holonomy_strength();
        let curv = ecology.curvature_field.strength();
        let align = ecology.alignment_field.strength();

        hol.abs() <= self.max_holonomy &&
        curv.abs() <= self.max_ecology_curvature &&
        align.abs() <= self.max_ecology_alignment &&
        rule.curvature_bias.abs() <= self.max_rule_bias &&
        rule.alignment_bias.abs() <= self.max_rule_bias &&
        rule.holonomy_bias.abs() <= self.max_rule_bias
    }
}
