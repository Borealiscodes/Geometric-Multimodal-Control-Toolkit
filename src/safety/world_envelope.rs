use crate::worlds::{SoftWorld, alignment_score};

/// WorldEnvelope enforces holonomy and alignment constraints.
pub struct WorldEnvelope {
    world: SoftWorld,
}

impl WorldEnvelope {
    pub fn new() -> Self {
        WorldEnvelope {
            world: SoftWorld::new(),
        }
    }

    pub fn enforce(&self, z1: &[f64], z2: &[f64]) -> bool {
        let hol = self.world.holonomy_loop();
        if hol.abs() > crate::HOLONOMY_TOLERANCE {
            return false;
        }

        let align = alignment_score(z1, z2);
        if align.abs() < crate::ALIGNMENT_TOLERANCE {
            return false;
        }

        true
    }
}
