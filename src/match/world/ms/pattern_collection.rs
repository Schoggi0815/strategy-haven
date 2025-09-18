use crate::r#match::world::ms::{
    constraint::{Constraint, ConstraintDirection},
    constraint_collection::ConstraintCollection,
    pattern::Pattern,
};

pub struct PatternCollection {
    pub patterns: Vec<Pattern>,
    pub pattern_counts: Vec<usize>,
}

impl PatternCollection {
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
            pattern_counts: Vec::new(),
        }
    }

    pub fn get(&self, pattern_id: usize) -> &Pattern {
        &self.patterns[pattern_id]
    }

    pub fn get_count(&self, pattern_id: usize) -> usize {
        self.pattern_counts[pattern_id]
    }

    pub fn add_or_get(&mut self, pattern: Pattern) -> usize {
        if let Some((id, _)) = self
            .patterns
            .iter()
            .enumerate()
            .find(|(_, p)| **p == pattern)
        {
            self.pattern_counts[id] += 1;
            id
        } else {
            let id = self.patterns.len();
            self.patterns.push(pattern);
            self.pattern_counts.push(1);
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
                ConstraintDirection::Top => ConstraintDirection::Left,
                ConstraintDirection::Right => ConstraintDirection::Top,
                ConstraintDirection::Bottom => ConstraintDirection::Right,
                ConstraintDirection::Left => ConstraintDirection::Bottom,
                ConstraintDirection::TopLeft => ConstraintDirection::BottomLeft,
                ConstraintDirection::TopRight => ConstraintDirection::TopLeft,
                ConstraintDirection::BottomLeft => ConstraintDirection::BottomRight,
                ConstraintDirection::BottomRight => ConstraintDirection::TopRight,
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
        let flipped_x = pattern.flip_x();
        let flipped_x_id = self.add_or_get(flipped_x.clone());
        let flipped_y = pattern.flip_y();
        let flipped_y_id = self.add_or_get(flipped_y.clone());
        let flipped_xy = flipped_x.flip_y();
        let flipped_xy_id = self.add_or_get(flipped_xy.clone());

        let mut new_constraints = Vec::new();

        for constraint in constraint_collection
            .constraints
            .iter()
            .filter(|constraint| constraint.pattern_a_id == pattern_id)
        {
            let other_flipped_x = self.patterns[constraint.pattern_b_id].flip_x();
            let other_flipped_x_id = self.add_or_get(other_flipped_x.clone());
            let other_flipped_y = self.patterns[constraint.pattern_b_id].flip_y();
            let other_flipped_y_id = self.add_or_get(other_flipped_y);
            let other_flipped_xy = other_flipped_x.flip_y();
            let other_flipped_xy_id = self.add_or_get(other_flipped_xy);

            let flipped_direction_x = constraint.direction.flip_x();
            let flipped_direction_y = constraint.direction.flip_y();
            let flipped_direction_xy = constraint.direction.reverse();

            let new_constraint_x = Constraint {
                pattern_a_id: flipped_x_id,
                pattern_b_id: other_flipped_x_id,
                direction: flipped_direction_x,
            };

            let new_constraint_x_back = Constraint {
                pattern_a_id: other_flipped_x_id,
                pattern_b_id: flipped_x_id,
                direction: flipped_direction_x.reverse(),
            };

            let new_constraint_y = Constraint {
                pattern_a_id: flipped_y_id,
                pattern_b_id: other_flipped_y_id,
                direction: flipped_direction_y,
            };

            let new_constraint_y_back = Constraint {
                pattern_a_id: other_flipped_y_id,
                pattern_b_id: flipped_y_id,
                direction: flipped_direction_y.reverse(),
            };

            let new_constraint_xy = Constraint {
                pattern_a_id: flipped_xy_id,
                pattern_b_id: other_flipped_xy_id,
                direction: flipped_direction_xy,
            };

            let new_constraint_xy_back = Constraint {
                pattern_a_id: other_flipped_xy_id,
                pattern_b_id: flipped_xy_id,
                direction: flipped_direction_xy.reverse(),
            };

            new_constraints.push(new_constraint_x);
            new_constraints.push(new_constraint_x_back);
            new_constraints.push(new_constraint_y);
            new_constraints.push(new_constraint_y_back);
            new_constraints.push(new_constraint_xy);
            new_constraints.push(new_constraint_xy_back);
        }

        new_constraints
            .into_iter()
            .for_each(|nc| constraint_collection.add(nc));

        (flipped_x, flipped_x_id)
    }
}
