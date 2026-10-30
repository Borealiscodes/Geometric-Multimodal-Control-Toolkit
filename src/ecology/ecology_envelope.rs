use crate::ecology::{ecology_curvature::EcologyCurvature, ecology_alignment::EcologyAlignment};

/// EcologyEnvelope enforces ecology-level stability.
/// This is the safety layer for Phase 8 ecology signals.
pub struct EcologyEnvelope {
    pub max_curvature: f64,
    pub max_alignment: f64,
}

impl EcologyEnvelope {
    pub fn new(max_curvature: f64, max_alignment: f64) -> Self {
        EcologyEnvelope {
            max_curvature,
            max_alignment,
        }
    }

    pub fn enforce(&self, curv: &EcologyCurvature, align: &EcologyAlignment) -> bool {
        curv.strength().abs() <= self.max_curvature &&
        align.strength().abs() <= self.max_alignment
    }
}
