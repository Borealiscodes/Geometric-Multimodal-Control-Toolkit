/// WorldRule represents a single world-level constraint.
/// Rules are generated from ecology and holonomy signals.
#[derive(Clone)]
pub struct WorldRule {
    pub weight: f64,
    pub curvature_bias: f64,
    pub alignment_bias: f64,
    pub holonomy_bias: f64,
}

impl WorldRule {
    pub fn new(weight: f64, curv: f64, align: f64, hol: f64) -> Self {
        WorldRule {
            weight,
            curvature_bias: curv,
            alignment_bias: align,
            holonomy_bias: hol,
        }
    }
}
