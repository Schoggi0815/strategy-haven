use crate::r#match::world::ms::constraint::Constraint;

pub struct ConstraintCollection {
    constraints: Vec<Constraint>,
}

impl ConstraintCollection {
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
        }
    }

    pub fn add(&mut self, constraint: Constraint) {
        if !self.constraints.contains(&constraint) {
            self.constraints.push(constraint);
        }
    }

    pub fn exists(&self, constraint: &Constraint) -> bool {
        self.constraints.contains(&constraint)
    }
}
