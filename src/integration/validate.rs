use crate::integration::integration_check;

/// validate_runtime is a simple wrapper used by Phase 7 to ensure
/// Phase 6 stability before applying operator safety envelopes.
pub fn validate_runtime() -> bool {
    let z1 = vec![0.0, 0.1, 0.2, 0.3];
    let z2 = vec![0.3, 0.2, 0.1, 0.0];

    integration_check(&z1, &z2)
}
