use gmc::safety::routing_envelope::RoutingEnvelope;
use gmc::geometry::{compute_hessian, collapse_detected};

#[test]
fn routing_envelope_avoids_collapse_surface() {
    let env = RoutingEnvelope::new();

    // Vector designed to collapse after routing
    let z = vec![1.0, 1.0, 1.0];

    let routed = env.route(&z);
    let h = compute_hessian(&routed);

    assert!(!collapse_detected(&h));
}
