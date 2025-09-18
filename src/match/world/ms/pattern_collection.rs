use itertools::Itertools;

use crate::r#match::world::ms::{
    constraint_direction::{ConstraintDirection, LAST_CONSTRAINT_DIRECTION},
    pattern::Pattern,
};

pub struct PatternCollection {
    pub patterns: Vec<Pattern>,
    pub pattern_counts: Vec<usize>,
    pub pattern_supports: Vec<Vec<Vec<usize>>>,
}

impl PatternCollection {
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
            pattern_counts: Vec::new(),
            pattern_supports: Vec::new(),
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
            self.pattern_supports.push(
                (0..=LAST_CONSTRAINT_DIRECTION as usize)
                    .map(|_| Vec::new())
                    .collect_vec(),
            );
            id
        }
    }

    pub fn add_support(&mut self, from_id: usize, direction: ConstraintDirection, to_id: usize) {
        if !self.pattern_supports[from_id][direction as usize].contains(&to_id) {
            self.pattern_supports[from_id][direction as usize].push(to_id);
        }
    }

    pub fn get_all_ids(&self) -> impl Iterator<Item = usize> {
        (0..self.patterns.len()).into_iter()
    }

    pub fn add_rotations(&mut self) {
        for (pattern_id, pattern) in self.patterns.clone().iter().enumerate() {
            let (rotated, rotated_id) = self.add_rotations_of_pattern(pattern, pattern_id);
            let (rotated, rotated_id) = self.add_rotations_of_pattern(&rotated, rotated_id);
            self.add_rotations_of_pattern(&rotated, rotated_id);
        }
    }

    pub fn add_flips(&mut self) {
        for (pattern_id, pattern) in self.patterns.clone().iter().enumerate() {
            self.add_flip_of_pattern(pattern, pattern_id);
        }
    }

    fn add_rotations_of_pattern(
        &mut self,
        pattern: &Pattern,
        pattern_id: usize,
    ) -> (Pattern, usize) {
        let rotated = pattern.get_rotation();
        let rotated_id = self.add_or_get(rotated.clone());

        for (constraint_direction, to_id) in self.pattern_supports[pattern_id]
            .iter()
            .enumerate()
            .flat_map(|(direction, supports)| {
                supports
                    .iter()
                    .map(move |to_id| (ConstraintDirection::from(direction), *to_id))
            })
            .collect_vec()
        {
            let other_rotated = self.patterns[to_id].get_rotation();
            let other_rotated_id = self.add_or_get(other_rotated);

            let rotated_direction = match constraint_direction {
                ConstraintDirection::Top => ConstraintDirection::Left,
                ConstraintDirection::Right => ConstraintDirection::Top,
                ConstraintDirection::Bottom => ConstraintDirection::Right,
                ConstraintDirection::Left => ConstraintDirection::Bottom,
                ConstraintDirection::TopLeft => ConstraintDirection::BottomLeft,
                ConstraintDirection::TopRight => ConstraintDirection::TopLeft,
                ConstraintDirection::BottomLeft => ConstraintDirection::BottomRight,
                ConstraintDirection::BottomRight => ConstraintDirection::TopRight,
            };

            self.add_support(rotated_id, rotated_direction, other_rotated_id);
            self.add_support(other_rotated_id, rotated_direction.reverse(), rotated_id);
        }

        (rotated, rotated_id)
    }

    fn add_flip_of_pattern(&mut self, pattern: &Pattern, pattern_id: usize) {
        let flipped_x = pattern.flip_x();
        let flipped_x_id = self.add_or_get(flipped_x.clone());
        let flipped_y = pattern.flip_y();
        let flipped_y_id = self.add_or_get(flipped_y.clone());
        let flipped_xy = flipped_x.flip_y();
        let flipped_xy_id = self.add_or_get(flipped_xy.clone());

        for (constraint_direction, to_id) in self.pattern_supports[pattern_id]
            .iter()
            .enumerate()
            .flat_map(|(direction, supports)| {
                supports
                    .iter()
                    .map(move |to_id| (ConstraintDirection::from(direction), *to_id))
            })
            .collect_vec()
        {
            let other_flipped_x = self.patterns[to_id].flip_x();
            let other_flipped_x_id = self.add_or_get(other_flipped_x.clone());
            let other_flipped_y = self.patterns[to_id].flip_y();
            let other_flipped_y_id = self.add_or_get(other_flipped_y);
            let other_flipped_xy = other_flipped_x.flip_y();
            let other_flipped_xy_id = self.add_or_get(other_flipped_xy);

            let flipped_direction_x = constraint_direction.flip_x();
            let flipped_direction_y = constraint_direction.flip_y();
            let flipped_direction_xy = constraint_direction.reverse();

            self.add_support(flipped_x_id, flipped_direction_x, other_flipped_x_id);
            self.add_support(
                other_flipped_x_id,
                flipped_direction_x.reverse(),
                flipped_x_id,
            );

            self.add_support(flipped_y_id, flipped_direction_y, other_flipped_y_id);
            self.add_support(
                other_flipped_y_id,
                flipped_direction_y.reverse(),
                flipped_y_id,
            );

            self.add_support(flipped_xy_id, flipped_direction_xy, other_flipped_xy_id);
            self.add_support(
                other_flipped_xy_id,
                flipped_direction_xy.reverse(),
                flipped_xy_id,
            );
        }
    }
}
