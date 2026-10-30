use crate::operators::{FusionOperator, RoutingOperator, CollapseGuard};
use crate::geometry::{curvature, compute_hessian, collapse_detected};
use crate::spectral::{eigenvalues, spectral_drift};
use crate::worlds::{SoftWorld, alignment_score};

/// Integration check for the full C-layer pipeline.
/// This ensures that all modules interact correctly and produce stable signals.
pub fn integration_check(z1: &[f64], z2: &[f64]) -> bool {
    // 1. Fusion
    let fusion = FusionOperator::new();
    let fused = fusion.apply(z1, z2);

    // 2. Routing
    let routing = RoutingOperator::new();
    let routed = routing.route(&fused);

    // 3. Collapse Guard
    let guard = CollapseGuard::new();
    if !guard.check(&routed) {
        return false;
    }

    // 4. Geometry
    let curv = curvature(&routed);
    let hessian = compute_hessian(&routed);
    if collapse_detected(&hessian) {
        return false;
    }

    // 5. Spectral
    let eigs = eigenvalues(&hessian);
    if eigs.iter().any(|e| *e < 0.0) {
        return false;
    }

    let drift = spectral_drift(&fused, &routed);
    if drift > crate::DRIFT_TOLERANCE {
        return false;
    }

    // 6. Soft-World
    let world = SoftWorld::new();
    let hol = world.holonomy_loop();
    if hol.abs() > crate::HOLONOMY_TOLERANCE {
        return false;
    }

    let align = alignment_score(&fused, &routed);
    if align.abs() < crate::ALIGNMENT_TOLERANCE {
        return false;
    }

    true
}
