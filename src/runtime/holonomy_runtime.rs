use crate::worlds::SoftWorld;
use crate::runtime::holonomy_field::HolonomyField;

/// HolonomyRuntime manages holonomy accumulation and decay.
/// It is the primary Phase 8 runtime module.
pub struct HolonomyRuntime {
    world: SoftWorld,
    field: HolonomyField,
}

impl HolonomyRuntime {
    pub fn new(decay_rate: f64) -> Self {
        HolonomyRuntime {
            world: SoftWorld::new(),
            field: HolonomyField::new(decay_rate),
        }
    }

    /// Update holonomy field using world holonomy loop.
    pub fn update(&mut self) {
        let hol = self.world.holonomy_loop();
        self.field.accumulate(hol);
        self.field.decay();
    }

    /// Return holonomy field strength.
    pub fn holonomy_strength(&self) -> f64 {
        self.field.strength()
    }
}
