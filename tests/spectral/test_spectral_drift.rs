use gmc::spectral::spectral_drift;

#[test]
fn spectral_drift_is_defined() {
    let d = spectral_drift(&[0.0], &[1.0]);
    assert!(d.is_finite());
}
