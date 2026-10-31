use serde::{Serialize, Deserialize};
use crate::geometry::collapse_surface::{CollapseSurface, Point3};
use crate::io::{load_seed_geometry, load_collapse_proximity, save_surface};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Batch21Output {
    pub surface: CollapseSurface,
}

pub fn run_batch21(seed_path: &str, proximity_path: &str, output_path: &str) -> Batch21Output {
    let seed: Vec<Point3> = load_seed_geometry(seed_path);
    let proximity: Vec<f64> = load_collapse_proximity(proximity_path);

    let surface = CollapseSurface::generate(&seed, &proximity);

    save_surface(output_path, &surface);

    Batch21Output { surface }
}
