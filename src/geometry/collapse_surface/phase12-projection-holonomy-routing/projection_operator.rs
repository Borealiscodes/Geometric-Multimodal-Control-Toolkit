pub struct HolonomyProjectionOperator;

impl HolonomyProjectionOperator {

    /// Phase 12B: Project transported quantities into manifold regions.
    ///
    /// transported: values from Phase 11
    /// paths: holonomy paths selected by Phase 12B
    ///
    /// Output: projected field with region-level assignments
    pub fn project<T: Copy>(
        transported: &Vec<T>,
        paths: &Vec<Vec<usize>>,
        manifold_size: usize
    ) -> Vec<Option<T>> {

        let mut field = vec![None; manifold_size];

        for (i, path) in paths.iter().enumerate() {
            // Project to the *last* region in the path (real routing)
            if let Some(&region) = path.last() {
                field[region] = Some(transported[i]);
            }
        }

        field
    }
}
