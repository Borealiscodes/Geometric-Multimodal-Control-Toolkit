use gmc::safety::collapse_envelope::CollapseEnvelope;
use gmc::geometry::compute_hessian;

#[test]
fn collapse_envelope_nudges_away_from_collapse() {
    let env = CollapseEnvelope::new();

    // Construct a vector with zero gradient → Hessian degeneracy
    let z = vec![1.0, 1.0, 1.0];

    let corrected = env.enforce(&z);
    let h = compute_hessian(&corrected);

    assert!(!gmc::geometry::collapse_detected(&h));
}
