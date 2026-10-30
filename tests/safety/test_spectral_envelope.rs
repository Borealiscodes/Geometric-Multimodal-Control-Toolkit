use gmc::safety::spectral_envelope::SpectralEnvelope;
use gmc::geometry::compute_hessian;

#[test]
fn spectral_envelope_clamps_negative_eigenvalues() {
    let env = SpectralEnvelope::new();

    let z_prev = vec![0.0, 0.1, 0.2];
    let z_next = vec![-1.0, -2.0, -3.0]; // intentionally unstable
    let h = compute_hessian(&z_next);

    let corrected = env.enforce(&z_prev, &z_next, &h);

    assert!(corrected.iter().all(|v| *v >= 0.0));
}

#[test]
fn spectral_envelope_damps_high_drift() {
    let env = SpectralEnvelope::new();

    let z_prev = vec![0.0, 0.0, 0.0];
    let z_next = vec![10.0, 10.0, 10.0]; // huge drift

    let h = compute_hessian(&z_next);
    let corrected = env.enforce(&z_prev, &z_next, &h);

    let drift = gmc::spectral::spectral_drift(&z_prev, &corrected);
    assert!(drift <= gmc::DRIFT_TOLERANCE);
}
