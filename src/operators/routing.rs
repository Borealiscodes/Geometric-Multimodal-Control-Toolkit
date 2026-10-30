use crate::worlds::alignment_score;

/// RoutingOperator performs holonomy-aware routing by nudging the vector
/// toward alignment with world geometry. This prevents routing drift.
pub struct RoutingOperator {
    pub step_size: f64,
}

impl RoutingOperator {
    pub fn new() -> Self {
        RoutingOperator { step_size: 0.1 }
    }

    /// Routing:
    ///   z_routed = z + step_size * alignment_gradient
    /// Alignment gradient is approximated using alignment_score.
    pub fn route(&self, z: &[f64]) -> Vec<f64> {
        let mut routed = z.to_vec();

        // Simple alignment gradient approximation:
        // Compare z to a slightly shifted version of itself.
        let mut shifted = Vec::with_capacity(z.len());
        for i in 0..z.len() {
            shifted.push(z[i] + 0.01);
        }

        let score = alignment_score(z, &shifted);

        // Apply routing step
        for i in 0..routed.len() {
            routed[i] += self.step_size * score;
        }

        routed
    }
}
