use crate::r#match::world::ms::{
    constraint::{Constraint, ConstraintDirection},
    constraint_collection::ConstraintCollection,
    pattern::Pattern,
};

pub struct PatternCollection {
    pub patterns: Vec<Pattern>,
}

impl PatternCollection {
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    pub fn get(&self, pattern_id: usize) -> &Pattern {
        &self.patterns[pattern_id]
    }

    pub fn add_or_get(&mut self, pattern: Pattern) -> usize {
        if let Some((id, _)) = self
            .patterns
            .iter()
            .enumerate()
            .find(|(_, p)| **p == pattern)
        {
            id
        } else {
            let id = self.patterns.len();
            self.patterns.push(pattern);
            id
        }
    }

    pub fn get_all_ids(&self) -> impl Iterator<Item = usize> {
        (0..self.patterns.len()).into_iter()
    }

    pub fn add_rotations(&mut self, constraint_collection: &mut ConstraintCollection) {
        for (pattern_id, pattern) in self.patterns.clone().iter().enumerate() {
            let (rotated, rotated_id) =
                self.add_rotations_of_pattern(constraint_collection, pattern, pattern_id);
            let (rotated, rotated_id) =
                self.add_rotations_of_pattern(constraint_collection, &rotated, rotated_id);
            self.add_rotations_of_pattern(constraint_collection, &rotated, rotated_id);
        }
    }

    pub fn add_flips(&mut self, constraint_collection: &mut ConstraintCollection) {
        for (pattern_id, pattern) in self.patterns.clone().iter().enumerate() {
            self.add_flip_of_pattern(constraint_collection, pattern, pattern_id);
        }
    }

    fn add_rotations_of_pattern(
        &mut self,
        constraint_collection: &mut ConstraintCollection,
        pattern: &Pattern,
        pattern_id: usize,
    ) -> (Pattern, usize) {
        let rotated = pattern.get_rotation();
        let rotated_id = self.add_or_get(rotated.clone());

        let mut new_constraints = Vec::new();

        for constraint in constraint_collection
            .constraints
            .iter()
            .filter(|constraint| constraint.pattern_a_id == pattern_id)
        {
            let other_rotated = self.patterns[constraint.pattern_b_id].get_rotation();
            let other_rotated_id = self.add_or_get(other_rotated);

            let rotated_direction = match constraint.direction {
                ConstraintDirection::Top => ConstraintDirection::Right,
                ConstraintDirection::Right => ConstraintDirection::Bottom,
                ConstraintDirection::Bottom => ConstraintDirection::Left,
                ConstraintDirection::Left => ConstraintDirection::Top,
            };

            let new_constraint = Constraint {
                pattern_a_id: rotated_id,
                pattern_b_id: other_rotated_id,
                direction: rotated_direction,
            };

            let new_constraint_back = Constraint {
                pattern_a_id: other_rotated_id,
                pattern_b_id: rotated_id,
                direction: rotated_direction.reverse(),
            };

            new_constraints.push(new_constraint);
            new_constraints.push(new_constraint_back);
        }

        new_constraints
            .into_iter()
            .for_each(|nc| constraint_collection.add(nc));

        (rotated, rotated_id)
    }

    fn add_flip_of_pattern(
        &mut self,
        constraint_collection: &mut ConstraintCollection,
        pattern: &Pattern,
        pattern_id: usize,
    ) -> (Pattern, usize) {
        let flipped = pattern.flip_x();
        let flipped_id = self.add_or_get(flipped.clone());

        let mut new_constraints = Vec::new();

        for constraint in constraint_collection
            .constraints
            .iter()
            .filter(|constraint| constraint.pattern_a_id == pattern_id)
        {
            let other_flipped = self.patterns[constraint.pattern_b_id].flip_x();
            let other_flipped_id = self.add_or_get(other_flipped);

            let flipped_direction = constraint.direction.reverse();

            let new_constraint = Constraint {
                pattern_a_id: flipped_id,
                pattern_b_id: other_flipped_id,
                direction: flipped_direction,
            };

            let new_constraint_back = Constraint {
                pattern_a_id: other_flipped_id,
                pattern_b_id: flipped_id,
                direction: flipped_direction.reverse(),
            };

            new_constraints.push(new_constraint);
            new_constraints.push(new_constraint_back);
        }

        new_constraints
            .into_iter()
            .for_each(|nc| constraint_collection.add(nc));

        (flipped, flipped_id)
    }
}
