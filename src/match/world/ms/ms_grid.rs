use itertools::{FoldWhile, Itertools};

use crate::r#match::world::{
    ms::{
        constraint::{Constraint, ConstraintDirection},
        constraint_collection::ConstraintCollection,
        pattern::Pattern,
        pattern_collection::PatternCollection,
    },
    tile_grid::TileGrid,
};

pub struct MSGrid {
    pattern_collection: PatternCollection,
    constraint_collection: ConstraintCollection,
    super_grid: Vec<Vec<Vec<usize>>>,
    grid_size: [usize; 2],
}

impl MSGrid {
    pub fn from_tile_grid(
        tile_grid: &TileGrid,
        pattern_size_x: usize,
        pattern_size_y: usize,
        grid_x_size: usize,
        grid_y_size: usize,
    ) -> Self {
        let mut pattern_collection = PatternCollection::new();
        let mut constraint_collection = ConstraintCollection::new();

        let mut pattern_ids = Vec::new();

        for x in 0..=tile_grid.grid_size[0] - pattern_size_x {
            pattern_ids.push(Vec::new());

            for y in 0..=tile_grid.grid_size[1] - pattern_size_y {
                let pattern = Pattern {
                    size: [pattern_size_x, pattern_size_y],
                    tiles: tile_grid
                        .data
                        .iter()
                        .skip(x)
                        .take(pattern_size_x)
                        .map(|column| {
                            column
                                .iter()
                                .skip(y)
                                .take(pattern_size_y)
                                .cloned()
                                .collect_vec()
                        })
                        .collect_vec(),
                };

                let pattern_id = pattern_collection.add_or_get(pattern);
                pattern_ids[x].push(pattern_id);

                if x > 2 {
                    let left_constraint = Constraint {
                        direction: ConstraintDirection::Left,
                        pattern_a_id: pattern_id,
                        pattern_b_id: pattern_ids[x - 3][y],
                    };

                    let right_constraint = Constraint {
                        direction: ConstraintDirection::Right,
                        pattern_a_id: pattern_ids[x - 3][y],
                        pattern_b_id: pattern_id,
                    };

                    constraint_collection.add(left_constraint);
                    constraint_collection.add(right_constraint);

                    if y > 2 {
                        let top_left_constraint = Constraint {
                            direction: ConstraintDirection::TopLeft,
                            pattern_a_id: pattern_id,
                            pattern_b_id: pattern_ids[x - 3][y - 3],
                        };

                        let bottom_right_constraint = Constraint {
                            direction: ConstraintDirection::BottomRight,
                            pattern_a_id: pattern_ids[x - 3][y - 3],
                            pattern_b_id: pattern_id,
                        };

                        constraint_collection.add(top_left_constraint);
                        constraint_collection.add(bottom_right_constraint);
                    }

                    if y < tile_grid.grid_size[1] - pattern_size_y - 2 {
                        let bottom_left_constraint = Constraint {
                            direction: ConstraintDirection::BottomLeft,
                            pattern_a_id: pattern_id,
                            pattern_b_id: pattern_ids[x - 3][y + 3],
                        };

                        let top_right_constraint = Constraint {
                            direction: ConstraintDirection::TopRight,
                            pattern_a_id: pattern_ids[x - 3][y + 3],
                            pattern_b_id: pattern_id,
                        };

                        constraint_collection.add(bottom_left_constraint);
                        constraint_collection.add(top_right_constraint);
                    }
                }

                if y > 2 {
                    let top_constraint = Constraint {
                        direction: ConstraintDirection::Top,
                        pattern_a_id: pattern_id,
                        pattern_b_id: pattern_ids[x][y - 3],
                    };

                    let bottom_constraint = Constraint {
                        direction: ConstraintDirection::Bottom,
                        pattern_a_id: pattern_ids[x][y - 3],
                        pattern_b_id: pattern_id,
                    };

                    constraint_collection.add(top_constraint);
                    constraint_collection.add(bottom_constraint);
                }
            }
        }

        pattern_collection.add_rotations(&mut constraint_collection);
        pattern_collection.add_flips(&mut constraint_collection);

        // for (id, pattern) in pattern_collection.patterns.iter().enumerate() {
        //     println!("Pattern with id {}:", id);
        //     println!("{}", pattern.get_grid());
        // }

        let grid = (0..grid_x_size)
            .map(|_| {
                (0..grid_y_size)
                    .map(|_| pattern_collection.get_all_ids().collect_vec())
                    .collect_vec()
            })
            .collect_vec();

        Self {
            pattern_collection,
            constraint_collection,
            super_grid: grid,
            grid_size: [grid_x_size, grid_y_size],
        }
    }

    pub fn collapse_grid(&mut self) {
        const SUBSET_SIZE: usize = 5;

        for x in 0..self.grid_size[0] / SUBSET_SIZE {
            for y in 0..self.grid_size[1] / SUBSET_SIZE {
                let offset = [x * SUBSET_SIZE, y * SUBSET_SIZE];
                let size = [
                    SUBSET_SIZE.min(self.grid_size[0] - (x * SUBSET_SIZE)),
                    SUBSET_SIZE.min(self.grid_size[1] - (y * SUBSET_SIZE)),
                ];

                let mut result = false;
                let mut fail_count = 0;

                let before_state = self.super_grid.clone();

                while !result {
                    result = self.collapse_subset(offset, size);

                    if !result {
                        println!("Subset at [{}, {}] failed!", x, y);

                        fail_count += 1;
                        self.super_grid = before_state.clone();

                        if fail_count >= 20 {
                            println!("Failed to generate subset {} times, exiting", fail_count);
                            return;
                        }
                    }
                }
            }
        }
    }

    pub fn collapse_subset(&mut self, offset: [usize; 2], subset_size: [usize; 2]) -> bool {
        for x in offset[0]..offset[0] + subset_size[0] {
            for y in offset[1]..offset[1] + subset_size[1] {
                let possible_states = &self.super_grid[x][y];

                let counts = possible_states
                    .iter()
                    .map(|id| self.pattern_collection.get_count(*id))
                    .collect_vec();

                let total_count: usize = counts.iter().sum();
                let random_count = rand::random_range(0..total_count);

                let (_, random_state) = possible_states
                    .iter()
                    .enumerate()
                    .fold_while((0, 0), |(acc, _), (i, id)| {
                        let new_acc = acc + self.pattern_collection.get_count(*id);
                        if new_acc >= random_count {
                            FoldWhile::Done((new_acc, i))
                        } else {
                            FoldWhile::Continue((new_acc, 0))
                        }
                    })
                    .into_inner();

                let random_state = possible_states[random_state];

                self.super_grid[x][y] = vec![random_state];
                if !self.propagate_change_from(x, y) {
                    return false;
                }
            }
        }

        true
    }

    pub fn to_tile_grid(&self) -> TileGrid {
        let pattern_grids = self.super_grid.iter().map(|column| {
            column
                .iter()
                .map(|states| self.pattern_collection.get(states[0]).get_grid())
        });

        pattern_grids
            .map(|column| {
                column
                    .reduce(|acc, current| acc.chain_below(current))
                    .unwrap()
            })
            .reduce(|acc, current| acc.chain_right(current))
            .unwrap()
    }

    fn propagate_change_from(&mut self, origin_x: usize, origin_y: usize) -> bool {
        let mut worklist = Vec::new();

        if origin_x > 0 {
            worklist.push((
                [origin_x, origin_y],
                [origin_x - 1, origin_y],
                ConstraintDirection::Right,
            ));

            if origin_y > 0 {
                worklist.push((
                    [origin_x, origin_y],
                    [origin_x - 1, origin_y - 1],
                    ConstraintDirection::BottomRight,
                ));
            }

            if origin_y < self.grid_size[1] - 1 {
                worklist.push((
                    [origin_x, origin_y],
                    [origin_x - 1, origin_y + 1],
                    ConstraintDirection::TopRight,
                ));
            }
        }

        if origin_x < self.grid_size[0] - 1 {
            worklist.push((
                [origin_x, origin_y],
                [origin_x + 1, origin_y],
                ConstraintDirection::Left,
            ));

            if origin_y > 0 {
                worklist.push((
                    [origin_x, origin_y],
                    [origin_x + 1, origin_y - 1],
                    ConstraintDirection::BottomLeft,
                ));
            }

            if origin_y < self.grid_size[1] - 1 {
                worklist.push((
                    [origin_x, origin_y],
                    [origin_x + 1, origin_y + 1],
                    ConstraintDirection::TopLeft,
                ));
            }
        }

        if origin_y > 0 {
            worklist.push((
                [origin_x, origin_y],
                [origin_x, origin_y - 1],
                ConstraintDirection::Bottom,
            ));
        }

        if origin_y < self.grid_size[1] - 1 {
            worklist.push((
                [origin_x, origin_y],
                [origin_x, origin_y + 1],
                ConstraintDirection::Top,
            ));
        }

        while let Some((origin, target, constraint_direction)) = worklist.pop() {
            let patterns = &self.super_grid[target[0]][target[1]];

            let mut new_valids = Vec::new();
            let mut added_directions = Vec::new();

            for pattern_id in patterns {
                let origin_ids = &self.super_grid[origin[0]][origin[1]];

                if !origin_ids.iter().any(|other_id| {
                    self.constraint_collection.exists(&Constraint {
                        pattern_a_id: *pattern_id,
                        pattern_b_id: *other_id,
                        direction: constraint_direction,
                    })
                }) {
                    if constraint_direction != ConstraintDirection::Left
                        && target[0] > 0
                        && !added_directions.contains(&ConstraintDirection::Right)
                    {
                        worklist.push((
                            target,
                            [target[0] - 1, target[1]],
                            ConstraintDirection::Right,
                        ));
                        added_directions.push(ConstraintDirection::Right);
                    }

                    if constraint_direction != ConstraintDirection::Right
                        && target[0] < self.grid_size[0] - 1
                        && !added_directions.contains(&ConstraintDirection::Left)
                    {
                        worklist.push((
                            target,
                            [target[0] + 1, target[1]],
                            ConstraintDirection::Left,
                        ));
                        added_directions.push(ConstraintDirection::Left);
                    }

                    if constraint_direction != ConstraintDirection::Top
                        && target[1] > 0
                        && !added_directions.contains(&ConstraintDirection::Bottom)
                    {
                        worklist.push((
                            target,
                            [target[0], target[1] - 1],
                            ConstraintDirection::Bottom,
                        ));
                        added_directions.push(ConstraintDirection::Bottom);
                    }

                    if constraint_direction != ConstraintDirection::Bottom
                        && target[1] < self.grid_size[1] - 1
                        && !added_directions.contains(&ConstraintDirection::Top)
                    {
                        worklist.push((
                            target,
                            [target[0], target[1] + 1],
                            ConstraintDirection::Top,
                        ));
                        added_directions.push(ConstraintDirection::Top);
                    }

                    if constraint_direction != ConstraintDirection::TopLeft
                        && target[0] > 0
                        && target[1] > 0
                        && !added_directions.contains(&ConstraintDirection::BottomRight)
                    {
                        worklist.push((
                            target,
                            [target[0] - 1, target[1] - 1],
                            ConstraintDirection::BottomRight,
                        ));
                        added_directions.push(ConstraintDirection::BottomRight);
                    }

                    if constraint_direction != ConstraintDirection::TopRight
                        && target[0] < self.grid_size[0] - 1
                        && target[1] > 0
                        && !added_directions.contains(&ConstraintDirection::BottomLeft)
                    {
                        worklist.push((
                            target,
                            [target[0] + 1, target[1] - 1],
                            ConstraintDirection::BottomLeft,
                        ));
                        added_directions.push(ConstraintDirection::BottomLeft);
                    }

                    if constraint_direction != ConstraintDirection::BottomLeft
                        && target[0] > 0
                        && target[1] < self.grid_size[1] - 1
                        && !added_directions.contains(&ConstraintDirection::TopRight)
                    {
                        worklist.push((
                            target,
                            [target[0] - 1, target[1] + 1],
                            ConstraintDirection::TopRight,
                        ));
                        added_directions.push(ConstraintDirection::TopRight);
                    }

                    if constraint_direction != ConstraintDirection::BottomRight
                        && target[0] < self.grid_size[0] - 1
                        && target[1] < self.grid_size[1] - 1
                        && !added_directions.contains(&ConstraintDirection::TopLeft)
                    {
                        worklist.push((
                            target,
                            [target[0] + 1, target[1] + 1],
                            ConstraintDirection::TopLeft,
                        ));
                        added_directions.push(ConstraintDirection::TopLeft);
                    }
                } else {
                    new_valids.push(*pattern_id);
                }
            }

            if new_valids.len() == 0 {
                return false;
            }

            self.super_grid[target[0]][target[1]] = new_valids;
        }

        true
    }
}
