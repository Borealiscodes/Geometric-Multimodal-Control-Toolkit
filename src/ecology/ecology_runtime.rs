use crate::ecology::{
    ecology_curvature::EcologyCurvature,
    ecology_alignment::EcologyAlignment,
    ecology_envelope::EcologyEnvelope,
};
use crate::geometry::curvature;
use crate::worlds::alignment_score;

/// EcologyRuntime integrates ecology curvature and alignment fields.
/// This is the core of the Projection-Seed Ecology Layer.
pub struct EcologyRuntime {
    pub curvature_field: EcologyCurvature,
    pub alignment_field: EcologyAlignment,
    pub envelope: EcologyEnvelope,
}

impl EcologyRuntime {
    pub fn new(curv_decay: f64, align_decay: f64, max_curv: f64, max_align: f64) -> Self {
        EcologyRuntime {
            curvature_field: EcologyCurvature::new(curv_decay),
            alignment_field: EcologyAlignment::new(align_decay),
            envelope: EcologyEnvelope::new(max_curv, max_align),
        }
    }

    /// Update ecology fields using world geometry and alignment.
    pub fn update(&mut self, z1: &[f64], z2: &[f64]) -> bool {
        let curv = curvature(z1);
        let align = alignment_score(z1, z2);

        self.curvature_field.accumulate(curv);
        self.alignment_field.accumulate(align);

        self.curvature_field.decay();
        self.alignment_field.decay();

        self.envelope.enforce(&self.curvature_field, &self.alignment_field)
    }
}
