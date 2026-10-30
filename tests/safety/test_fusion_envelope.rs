use gmc::safety::fusion_envelope::FusionEnvelope;
use gmc::geometry::curvature;

#[test]
fn fusion_envelope_bounds_curvature() {
    let env = FusionEnvelope::new();

    let z1 = vec![0.0, 100.0, -100.0];
    let z2 = vec![50.0, -50.0, 25.0];

    let fused = env.apply(&z1, &z2);
    let curv = curvature(&fused);

    assert!(curv.abs() <= gmc::CURVATURE_BOUND);
}
