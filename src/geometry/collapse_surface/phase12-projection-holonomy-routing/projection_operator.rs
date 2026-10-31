pub struct HolonomyProjectionOperator;

impl HolonomyProjectionOperator {

    /// Phase 12: Project transported quantities into manifold regions.
    ///
    /// transported: values from Phase 11 (normals, curvature, compatibility)
    /// paths: holonomy paths selected by HolonomyPathSelector
    ///
    /// Output: projected field over manifold regions
    pub fn project<T: Copy>(
        transported: &Vec<T>,
        paths: &Vec<Vec<usize>>,
        manifold_size: usize
    ) -> Vec<Option<T>> {

        let mut field = vec![None; manifold_size];

        for (i, path) in paths.iter().enumerate() {
            let region = path[0]; // placeholder: project to first region
            field[region] = Some(transported[i]);
        }

        field
    }
}
