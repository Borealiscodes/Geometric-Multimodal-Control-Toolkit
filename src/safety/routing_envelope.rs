use crate::operators::RoutingOperator;
use crate::geometry::{compute_hessian, collapse_detected};

/// RoutingEnvelope wraps RoutingOperator and prevents routing into collapse surfaces.
pub struct RoutingEnvelope {
    routing: RoutingOperator,
}

impl RoutingEnvelope {
    pub fn new() -> Self {
        RoutingEnvelope {
            routing: RoutingOperator::new(),
        }
    }

    pub fn route(&self, z: &[f64]) -> Vec<f64> {
        let routed = self.routing.route(z);

        let h = compute_hessian(&routed);
        if collapse_detected(&h) {
            // Nudge away from collapse
            return routed.iter().map(|v| v + 0.01).collect();
        }

        routed
    }
}
