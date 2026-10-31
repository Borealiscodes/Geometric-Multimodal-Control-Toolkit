use std::f32::consts::PI;

pub struct HolonomyTransportOperator;

impl HolonomyTransportOperator {

    /// Phase 11: Parallel transport of normals along holonomy paths.
    ///
    /// normals: collapse-surface normals (Phase 10)
    /// paths: holonomy traversal paths, each path is a sequence of manifold indices
    ///
    /// Output: transported normals, one per path
    pub fn transport_normals(
        normals: &Vec<[f32; 3]>,
        paths: &Vec<Vec<usize>>
    ) -> Vec<[f32; 3]> {

        let mut transported = Vec::new();

        for path in paths {
            // Take the starting normal
            let start_idx = path[0];
            let mut current = normals[start_idx];

            // Minimal holonomy kernel:
            // For each step along the path, apply a tiny rotation
            // This is a placeholder for the full holonomy connection.
            for _step in path.iter().skip(1) {
                current = Self::rotate_small(current);
            }

            // Normalize to preserve vector magnitude
            current = Self::normalize(current);

            transported.push(current);
        }

        transported
    }

    /// Minimal rotation kernel for holonomy transport.
    /// This is intentionally simple: a tiny rotation around Z-axis.
    /// Later phases will replace this with the full connection-based transport.
    fn rotate_small(v: [f32; 3]) -> [f32; 3] {
        let theta = 0.01 * PI; // small rotation
        let cos_t = theta.cos();
        let sin_t = theta.sin();

        [
            v[0] * cos_t - v[1] * sin_t,
            v[0] * sin_t + v[1] * cos_t,
            v[2]
        ]
    }

    /// Normalize a vector to unit length.
    fn normalize(v: [f32; 3]) -> [f32; 3] {
        let mag = (v[0]*v[0] + v[1]*v[1] + v[2]*v[2]).sqrt();
        if mag == 0.0 {
            return v;
        }
        [v[0]/mag, v[1]/mag, v[2]/mag]
    }
}
