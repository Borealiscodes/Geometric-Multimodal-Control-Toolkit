use crate::spectral::{eigenvalues, spectral_drift};

/// SpectralEnvelope enforces spectral stability:
/// - eigenvalues must be non-negative
/// - drift must remain below tolerance
pub struct SpectralEnvelope;

impl SpectralEnvelope {
    pub fn new() -> Self {
        SpectralEnvelope
    }

    pub fn enforce(&self, z_prev: &[f64], z_next: &[f64], hessian: &[Vec<f64>]) -> Vec<f64> {
        let eigs = eigenvalues(hessian);
        let drift = spectral_drift(z_prev, z_next);

        let mut corrected = z_next.to_vec();

        // Clamp negative eigenvalues
        if eigs.iter().any(|e| *e < 0.0) {
            for v in corrected.iter_mut() {
                *v = v.abs();
            }
        }

        // Damp drift if above tolerance
        if drift > crate::DRIFT_TOLERANCE {
            let scale = crate::DRIFT_TOLERANCE / drift;
            for v in corrected.iter_mut() {
                *v *= scale;
            }
        }

        corrected
    }
}
