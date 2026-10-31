use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollapseSurface {
    pub points: Vec<Point3>,
    pub normals: Vec<Vector3>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl CollapseSurface {
    pub fn generate(seed: &[Point3], proximity: &[f64]) -> Self {
        let mut points = Vec::new();
        let mut normals = Vec::new();

        for (i, p) in seed.iter().enumerate() {
            let scale = proximity[i];
            let surface_point = Point3 {
                x: p.x * (1.0 + scale),
                y: p.y * (1.0 + scale),
                z: p.z * (1.0 + scale),
            };

            let normal = Vector3 {
                x: p.x,
                y: p.y,
                z: p.z,
            };

            points.push(surface_point);
            normals.push(normal.normalize());
        }

        CollapseSurface { points, normals }
    }
}

impl Vector3 {
    pub fn normalize(&self) -> Self {
        let mag = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if mag == 0.0 {
            return Self { x: 0.0, y: 0.0, z: 0.0 };
        }
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }
}
