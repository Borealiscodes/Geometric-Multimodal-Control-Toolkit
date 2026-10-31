use crate::runtime::holonomy_runtime::HolonomyRuntime;
use crate::ecology::ecology_runtime::EcologyRuntime;
use crate::worlds::world_rule_synthesizer::WorldRuleSynthesizer;
use crate::worlds::world_rule_runtime::WorldRuleRuntime;
use crate::safety::ecology_envelope::EcologySafetyEnvelope;

/// EcologySafetyEngine orchestrates Phase 8 safety:
/// - updates holonomy runtime
/// - updates ecology runtime
/// - synthesizes world-rules
/// - enforces ecology-aware safety envelope
/// - applies world-rules if stable
pub struct EcologySafetyEngine {
    pub holonomy: HolonomyRuntime,
    pub ecology: EcologyRuntime,
    pub synthesizer: WorldRuleSynthesizer,
    pub rule_runtime: WorldRuleRuntime,
    pub envelope: EcologySafetyEnvelope,
}

impl EcologySafetyEngine {
    pub fn new(
        hol_decay: f64,
        curv_decay: f64,
        align_decay: f64,
        max_hol: f64,
        max_curv: f64,
        max_align: f64,
        max_rule_bias: f64,
    ) -> Self {
        EcologySafetyEngine {
            holonomy: HolonomyRuntime::new(hol_decay),
            ecology: EcologyRuntime::new(curv_decay, align_decay, max_curv, max_align),
            synthesizer: WorldRuleSynthesizer::new(),
            rule_runtime: WorldRuleRuntime::new(),
            envelope: EcologySafetyEnvelope::new(
                max_hol,
                max_curv,
                max_align,
                max_rule_bias,
            ),
        }
    }

    /// Perform one ecology-aware safety step.
    pub fn step(&mut self, z1: &[f64], z2: &[f64]) -> bool {
        // Update holonomy
        self.holonomy.update();

        // Update ecology
        let ecology_ok = self.ecology.update(z1, z2);

        // Synthesize world-rule
        let rule = self.synthesizer.synthesize(&self.ecology, &self.holonomy);

        // Enforce envelope
        let stable = ecology_ok && self.envelope.enforce(&self.holonomy, &self.ecology, &rule);

        if stable {
            self.rule_runtime.apply_rule(&rule);
        }

        stable
    }
}
