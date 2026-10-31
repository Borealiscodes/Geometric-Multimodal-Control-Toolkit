use serde::{Serialize, Deserialize};
use crate::geometry::collapse_alignment::AlignmentTensor;
use crate::geometry::collapse_compat::CompatibilityEnvelope;
use crate::io::{load_alignment, load_compatibility_matrix, save_compatibility};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Batch24Output {
    pub envelope: CompatibilityEnvelope,
    pub incompatible_regions: Vec<usize>,
}

pub fn run_batch24(alignment_path: &str, matrix_path: &str, output_path: &str) -> Batch24Output {
    let alignment: AlignmentTensor = load_alignment(alignment_path);
    let matrix: Vec<f64> = load_compatibility_matrix(matrix_path);

    let envelope = CompatibilityEnvelope::evaluate(&alignment, &matrix);
    let incompatible_regions = envelope.incompatible_regions();

    save_compatibility(output_path, &envelope);

    Batch24Output {
        envelope,
        incompatible_regions,
    }
}
