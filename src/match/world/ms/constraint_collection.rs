use rustc_hash::FxHashSet;

use crate::r#match::world::ms::constraint::Constraint;

pub struct ConstraintCollection {
    pub constraints: FxHashSet<Constraint>,
}

impl ConstraintCollection {
    pub fn new() -> Self {
        Self {
            constraints: FxHashSet::default(),
        }
    }

    pub fn add(&mut self, constraint: Constraint) {
        self.constraints.insert(constraint);
    }

    pub fn exists(&self, constraint: &Constraint) -> bool {
        self.constraints.contains(&constraint)
    }
}
