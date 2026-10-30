mod operators;
mod geometry;
mod spectral;
mod worlds;
mod integration;

#[test]
fn phase6_runtime_is_consistent() {
    // Basic vectors for pipeline validation
    let z1 = vec![0.0, 0.1, 0.2, 0.3];
    let z2 = vec![0.3, 0.2, 0.1, 0.0];

    // Run integration check
    let ok = integration::integration_check(&z1, &z2);

    // Phase 6 requires full runtime stability
    assert!(ok, "Phase 6 runtime stability check failed");
}
