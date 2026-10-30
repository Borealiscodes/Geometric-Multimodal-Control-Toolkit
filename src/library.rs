pub mod operators;
pub mod geometry;
pub mod spectral;
pub mod worlds;

// Shared constants for stability testing
pub const CURVATURE_BOUND: f64 = 10.0;
pub const HOLONOMY_TOLERANCE: f64 = 1e-3;
pub const MIN_HESSIAN_DET: f64 = 1e-6;
