use crate::geometry::curvature;

/// SoftWorld represents a lightweight geometric world model used for
/// holonomy and coherence checks. It does not encode full world rules yet;
/// Phase 7 will extend this into a full operator safety envelope.
pub struct SoftWorld {
    pub loop_step: f64,
}

impl SoftWorld {
    pub fn new() -> Self {
        SoftWorld { loop_step: 0.05 }
    }

    /// holonomy_loop computes a simple holonomy measure by walking a loop
    /// around a synthetic manifold defined by curvature. This is not a full
    /// holonomy operator — it is a runtime stability signal.
    ///
    /// Holonomy is approximated as:
    ///     H = Σ curvature(z_loop[i])
    ///
    /// where z_loop is a synthetic loop generated from loop_step.
    pub fn holonomy_loop(&self) -> f64 {
        let mut acc = 0.0;

        // Synthetic loop: 40 steps around a small circle-like manifold
        let mut z = vec![0.0];
        for _ in 0..40 {
            let next = z.last().unwrap() + self.loop_step;
            z.push(next);
        }

        // Accumulate curvature along the loop
        for i in 0..z.len() {
            acc += curvature(&z[i..]);
        }

        acc
    }
}
