use crate::runtime::holonomy_runtime::HolonomyRuntime;
use crate::ecology::ecology_runtime::EcologyRuntime;
use crate::worlds::world_rule_runtime::WorldRuleRuntime;

/// ProjectionSeed is the fundamental geometric primitive of Phase 9.
/// It is generated from holonomy, ecology curvature, ecology alignment,
/// and world-rule biases.
#[derive(Clone, Debug)]
pub struct ProjectionSeed {
    pub curvature: f64,
    pub holonomy_bias: f64,
    pub alignment_bias: f64,
    pub collapse_proximity: f64,
}

impl ProjectionSeed {
    pub fn new(
        curvature: f64,
        holonomy_bias: f64,
        alignment_bias: f64,
        collapse_proximity: f64,
    ) -> Self {
        ProjectionSeed {
            curvature,
            holonomy_bias,
            alignment_bias,
            collapse_proximity,
        }
    }

    /// Seeds respond to holonomy by adjusting curvature.
    pub fn apply_holonomy(&mut self, hol: f64) {
        self.curvature += hol * 0.1;
        self.holonomy_bias = hol;
    }

    /// Seeds respond to ecology alignment.
    pub fn apply_alignment(&mut self, align: f64) {
        self.alignment_bias = align;
    }

    /// Seeds respond to collapse proximity (Phase 10 will use this).
    pub fn set_collapse_proximity(&mut self, prox: f64) {
        self.collapse_proximity = prox;
    }
}
