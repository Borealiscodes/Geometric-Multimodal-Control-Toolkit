pub mod operators;
pub mod geometry;
pub mod spectral;
pub mod worlds;

// Global numeric conventions for Phase 6
pub const CURVATURE_BOUND: f64 = 10.0;       // max allowed curvature
pub const HOLONOMY_TOLERANCE: f64 = 1e-3;    // soft-world holonomy tolerance
pub const MIN_HESSIAN_DET: f64 = 1e-6;       // collapse threshold
pub const DRIFT_TOLERANCE: f64 = 0.05;       // spectral drift threshold
pub const ALIGNMENT_TOLERANCE: f64 = 0.1;    // multimodal alignment threshold
