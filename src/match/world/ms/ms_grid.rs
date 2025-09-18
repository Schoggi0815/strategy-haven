use itertools::{FoldWhile, Itertools};

use crate::r#match::world::{
    ms::{
        constraint_direction::{ConstraintDirection, LAST_CONSTRAINT_DIRECTION},
        pattern::Pattern,
        pattern_collection::PatternCollection,
    },
    tile_grid::TileGrid,
};

pub struct MSGrid {
    pattern_collection: PatternCollection,
    support_count_grid: Vec<Vec<Vec<Vec<usize>>>>,
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

                if x > (pattern_size_x - 1) {
                    pattern_collection.add_support(
                        pattern_id,
                        ConstraintDirection::Left,
                        pattern_ids[x - pattern_size_x][y],
                    );

                    pattern_collection.add_support(
                        pattern_ids[x - pattern_size_x][y],
                        ConstraintDirection::Right,
                        pattern_id,
                    );

                    if y > (pattern_size_y - 1) {
                        pattern_collection.add_support(
                            pattern_id,
                            ConstraintDirection::TopLeft,
                            pattern_ids[x - pattern_size_x][y - pattern_size_y],
                        );

                        pattern_collection.add_support(
                            pattern_ids[x - pattern_size_x][y - pattern_size_y],
                            ConstraintDirection::BottomRight,
                            pattern_id,
                        );
                    }

                    if y < tile_grid.grid_size[1] - pattern_size_y - (pattern_size_y - 1) {
                        pattern_collection.add_support(
                            pattern_id,
                            ConstraintDirection::BottomLeft,
                            pattern_ids[x - pattern_size_x][y + pattern_size_y],
                        );

                        pattern_collection.add_support(
                            pattern_ids[x - pattern_size_x][y + pattern_size_y],
                            ConstraintDirection::TopRight,
                            pattern_id,
                        );
                    }
                }

                if y > (pattern_size_y - 1) {
                    pattern_collection.add_support(
                        pattern_id,
                        ConstraintDirection::Top,
                        pattern_ids[x][y - pattern_size_y],
                    );

                    pattern_collection.add_support(
                        pattern_ids[x][y - pattern_size_y],
                        ConstraintDirection::Bottom,
                        pattern_id,
                    );
                }
            }
        }

        pattern_collection.add_rotations();
        pattern_collection.add_flips();

        // pattern_collection
        //     .pattern_supports
        //     .iter()
        //     .enumerate()
        //     .filter(|(_, directions)| {
        //         directions
        //             .iter()
        //             .any(|other_patterns| other_patterns.len() == 0)
        //     })
        //     .map(|(id, connections)| (pattern_collection.get(id), connections))
        //     .for_each(|(pattern, connections)| {
        //         println!("connections: {:?}", connections);
        //         println!("{}", pattern.get_grid())
        //     });

        // for (id, pattern) in pattern_collection.patterns.iter().enumerate() {
        //     println!("Pattern with id {}:", id);
        //     println!("{}", pattern.get_grid());
        // }

        let pattern_supports = pattern_collection
            .pattern_supports
            .iter()
            .map(|directions| {
                directions
                    .iter()
                    .map(|other_patterns| other_patterns.len())
                    .collect_vec()
            })
            .collect_vec();

        let support_count_grid = (0..grid_x_size)
            .map(|_| {
                (0..grid_y_size)
                    .map(|_| pattern_supports.clone())
                    .collect_vec()
            })
            .collect_vec();

        Self {
            pattern_collection,
            support_count_grid,
            grid_size: [grid_x_size, grid_y_size],
        }
    }

    pub fn collapse_grid(&mut self) {
        const SUBSET_SIZE: usize = 10;

        for x in 0..=self.grid_size[0] / SUBSET_SIZE {
            for y in 0..=self.grid_size[1] / SUBSET_SIZE {
                let offset = [x * SUBSET_SIZE, y * SUBSET_SIZE];
                let size = [
                    SUBSET_SIZE.min(self.grid_size[0] - (x * SUBSET_SIZE)),
                    SUBSET_SIZE.min(self.grid_size[1] - (y * SUBSET_SIZE)),
                ];

                let mut result = false;
                let mut fail_count = 0;

                let before_state = self.support_count_grid.clone();

                while !result {
                    result = self.collapse_subset(offset, size);

                    if !result {
                        println!("Subset at [{}, {}] failed!", x, y);

                        fail_count += 1;
                        self.support_count_grid = before_state.clone();

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
                let possible_states = &self.support_count_grid[x][y]
                    .iter()
                    .enumerate()
                    .filter(|(_, direction_counts)| direction_counts.iter().all(|count| *count > 0))
                    .map(|(id, _)| id)
                    .collect_vec();

                if possible_states.len() == 1 {
                    continue;
                }

                let counts = possible_states
                    .iter()
                    .map(|id| self.pattern_collection.get_count(*id))
                    .collect_vec();

                let total_count: usize = counts.iter().sum();
                let random_count = rand::random_range(0..total_count);

                let random_state = possible_states
                    .iter()
                    .fold_while((0, None), |(acc, _), id| {
                        let new_acc = acc + self.pattern_collection.get_count(*id);
                        if new_acc >= random_count {
                            FoldWhile::Done((new_acc, Some(*id)))
                        } else {
                            FoldWhile::Continue((new_acc, None))
                        }
                    })
                    .into_inner()
                    .1
                    .unwrap();

                // let random_state = possible_states[rand::random_range(0..possible_states.len())];

                // println!("Popped pattern for {}, {}:", x, y);
                // println!("{}", self.pattern_collection.get(random_state).get_grid());

                let removed_states = possible_states
                    .iter()
                    .filter(|state| **state != random_state)
                    .cloned()
                    .collect_vec();

                self.support_count_grid[x][y]
                    .iter_mut()
                    .enumerate()
                    .filter(|(pattern_id, _)| *pattern_id != random_state)
                    .for_each(|(_, direcion_counts)| {
                        *direcion_counts = (0..=LAST_CONSTRAINT_DIRECTION as usize)
                            .map(|_| 0)
                            .collect_vec();
                    });

                if !self.propagate_change_from_ac4(x, y, removed_states) {
                    return false;
                }

                // println!("Step {}, {}:", x, y);
                // println!("{}", self.to_tile_grid());
            }
        }

        true
    }

    pub fn to_tile_grid(&self) -> TileGrid {
        let pattern_id_grid = self.support_count_grid.iter().map(|column| {
            column.iter().map(|pattern_id_counts| {
                pattern_id_counts
                    .iter()
                    .enumerate()
                    .filter(|(_, direction_counts)| direction_counts.iter().all(|count| *count > 0))
                    .nth(0)
                    .unwrap()
                    .0
            })
        });

        let pattern_grids = pattern_id_grid
            .map(|column| column.map(|state| self.pattern_collection.get(state).get_grid()));

        pattern_grids
            .map(|column| {
                column
                    .reduce(|acc, current| acc.chain_below(current))
                    .unwrap()
            })
            .reduce(|acc, current| acc.chain_right(current))
            .unwrap()
    }

    fn propagate_change_from_ac4(
        &mut self,
        origin_x: usize,
        origin_y: usize,
        removed_states: Vec<usize>,
    ) -> bool {
        let mut inconcistencies = removed_states
            .iter()
            .map(|state| ([origin_x, origin_y], *state))
            .collect_vec();

        while let Some((pos, removed_pattern_id)) = inconcistencies.pop() {
            for (neighbour_pos, neighbour_direction) in self.get_neigbours(pos[0], pos[1]) {
                for affected_pattern_id in self.pattern_collection.pattern_supports
                    [removed_pattern_id][neighbour_direction.reverse() as usize]
                    .iter()
                {
                    if self.support_count_grid[neighbour_pos[0]][neighbour_pos[1]]
                        [*affected_pattern_id]
                        .iter()
                        .any(|count| *count == 0)
                    {
                        continue;
                    }

                    self.support_count_grid[neighbour_pos[0]][neighbour_pos[1]]
                        [*affected_pattern_id][neighbour_direction as usize] -= 1;

                    if self.support_count_grid[neighbour_pos[0]][neighbour_pos[1]]
                        [*affected_pattern_id][neighbour_direction as usize]
                        == 0
                    {
                        if !self.support_count_grid[neighbour_pos[0]][neighbour_pos[1]]
                            .iter()
                            .any(|direction_counts| direction_counts.iter().all(|count| *count > 0))
                        {
                            return false;
                        }

                        inconcistencies.push((neighbour_pos, *affected_pattern_id));
                    }
                }
            }
        }

        true
    }

    fn get_neigbours(
        &self,
        origin_x: usize,
        origin_y: usize,
    ) -> Vec<([usize; 2], ConstraintDirection)> {
        let mut constraints = Vec::new();

        if origin_x > 0 {
            constraints.push(([origin_x - 1, origin_y], ConstraintDirection::Right));

            if origin_y > 0 {
                constraints.push((
                    [origin_x - 1, origin_y - 1],
                    ConstraintDirection::BottomRight,
                ));
            }

            if origin_y < self.grid_size[1] - 1 {
                constraints.push(([origin_x - 1, origin_y + 1], ConstraintDirection::TopRight));
            }
        }

        if origin_x < self.grid_size[0] - 1 {
            constraints.push(([origin_x + 1, origin_y], ConstraintDirection::Left));

            if origin_y > 0 {
                constraints.push((
                    [origin_x + 1, origin_y - 1],
                    ConstraintDirection::BottomLeft,
                ));
            }

            if origin_y < self.grid_size[1] - 1 {
                constraints.push(([origin_x + 1, origin_y + 1], ConstraintDirection::TopLeft));
            }
        }

        if origin_y > 0 {
            constraints.push(([origin_x, origin_y - 1], ConstraintDirection::Bottom));
        }

        if origin_y < self.grid_size[1] - 1 {
            constraints.push(([origin_x, origin_y + 1], ConstraintDirection::Top));
        }

        constraints
    }

    // fn propagate_change_from(&mut self, origin_x: usize, origin_y: usize) -> bool {
    //     let mut worklist = Vec::new();

    //     if origin_x > 0 {
    //         worklist.push((
    //             [origin_x, origin_y],
    //             [origin_x - 1, origin_y],
    //             ConstraintDirection::Right,
    //         ));

    //         if origin_y > 0 {
    //             worklist.push((
    //                 [origin_x, origin_y],
    //                 [origin_x - 1, origin_y - 1],
    //                 ConstraintDirection::BottomRight,
    //             ));
    //         }

    //         if origin_y < self.grid_size[1] - 1 {
    //             worklist.push((
    //                 [origin_x, origin_y],
    //                 [origin_x - 1, origin_y + 1],
    //                 ConstraintDirection::TopRight,
    //             ));
    //         }
    //     }

    //     if origin_x < self.grid_size[0] - 1 {
    //         worklist.push((
    //             [origin_x, origin_y],
    //             [origin_x + 1, origin_y],
    //             ConstraintDirection::Left,
    //         ));

    //         if origin_y > 0 {
    //             worklist.push((
    //                 [origin_x, origin_y],
    //                 [origin_x + 1, origin_y - 1],
    //                 ConstraintDirection::BottomLeft,
    //             ));
    //         }

    //         if origin_y < self.grid_size[1] - 1 {
    //             worklist.push((
    //                 [origin_x, origin_y],
    //                 [origin_x + 1, origin_y + 1],
    //                 ConstraintDirection::TopLeft,
    //             ));
    //         }
    //     }

    //     if origin_y > 0 {
    //         worklist.push((
    //             [origin_x, origin_y],
    //             [origin_x, origin_y - 1],
    //             ConstraintDirection::Bottom,
    //         ));
    //     }

    //     if origin_y < self.grid_size[1] - 1 {
    //         worklist.push((
    //             [origin_x, origin_y],
    //             [origin_x, origin_y + 1],
    //             ConstraintDirection::Top,
    //         ));
    //     }

    //     while let Some((origin, target, constraint_direction)) = worklist.pop() {
    //         let patterns = &self.super_grid[target[0]][target[1]];

    //         let mut new_valids = Vec::new();
    //         let mut added_directions = Vec::new();

    //         for pattern_id in patterns {
    //             let origin_ids = &self.super_grid[origin[0]][origin[1]];

    //             if !origin_ids.iter().any(|other_id| {
    //                 self.constraint_collection.exists(&Constraint {
    //                     pattern_a_id: *pattern_id,
    //                     pattern_b_id: *other_id,
    //                     direction: constraint_direction,
    //                 })
    //             }) {
    //                 if constraint_direction != ConstraintDirection::Left
    //                     && target[0] > 0
    //                     && !added_directions.contains(&ConstraintDirection::Right)
    //                 {
    //                     worklist.push((
    //                         target,
    //                         [target[0] - 1, target[1]],
    //                         ConstraintDirection::Right,
    //                     ));
    //                     added_directions.push(ConstraintDirection::Right);
    //                 }

    //                 if constraint_direction != ConstraintDirection::Right
    //                     && target[0] < self.grid_size[0] - 1
    //                     && !added_directions.contains(&ConstraintDirection::Left)
    //                 {
    //                     worklist.push((
    //                         target,
    //                         [target[0] + 1, target[1]],
    //                         ConstraintDirection::Left,
    //                     ));
    //                     added_directions.push(ConstraintDirection::Left);
    //                 }

    //                 if constraint_direction != ConstraintDirection::Top
    //                     && target[1] > 0
    //                     && !added_directions.contains(&ConstraintDirection::Bottom)
    //                 {
    //                     worklist.push((
    //                         target,
    //                         [target[0], target[1] - 1],
    //                         ConstraintDirection::Bottom,
    //                     ));
    //                     added_directions.push(ConstraintDirection::Bottom);
    //                 }

    //                 if constraint_direction != ConstraintDirection::Bottom
    //                     && target[1] < self.grid_size[1] - 1
    //                     && !added_directions.contains(&ConstraintDirection::Top)
    //                 {
    //                     worklist.push((
    //                         target,
    //                         [target[0], target[1] + 1],
    //                         ConstraintDirection::Top,
    //                     ));
    //                     added_directions.push(ConstraintDirection::Top);
    //                 }

    //                 if constraint_direction != ConstraintDirection::TopLeft
    //                     && target[0] > 0
    //                     && target[1] > 0
    //                     && !added_directions.contains(&ConstraintDirection::BottomRight)
    //                 {
    //                     worklist.push((
    //                         target,
    //                         [target[0] - 1, target[1] - 1],
    //                         ConstraintDirection::BottomRight,
    //                     ));
    //                     added_directions.push(ConstraintDirection::BottomRight);
    //                 }

    //                 if constraint_direction != ConstraintDirection::TopRight
    //                     && target[0] < self.grid_size[0] - 1
    //                     && target[1] > 0
    //                     && !added_directions.contains(&ConstraintDirection::BottomLeft)
    //                 {
    //                     worklist.push((
    //                         target,
    //                         [target[0] + 1, target[1] - 1],
    //                         ConstraintDirection::BottomLeft,
    //                     ));
    //                     added_directions.push(ConstraintDirection::BottomLeft);
    //                 }

    //                 if constraint_direction != ConstraintDirection::BottomLeft
    //                     && target[0] > 0
    //                     && target[1] < self.grid_size[1] - 1
    //                     && !added_directions.contains(&ConstraintDirection::TopRight)
    //                 {
    //                     worklist.push((
    //                         target,
    //                         [target[0] - 1, target[1] + 1],
    //                         ConstraintDirection::TopRight,
    //                     ));
    //                     added_directions.push(ConstraintDirection::TopRight);
    //                 }

    //                 if constraint_direction != ConstraintDirection::BottomRight
    //                     && target[0] < self.grid_size[0] - 1
    //                     && target[1] < self.grid_size[1] - 1
    //                     && !added_directions.contains(&ConstraintDirection::TopLeft)
    //                 {
    //                     worklist.push((
    //                         target,
    //                         [target[0] + 1, target[1] + 1],
    //                         ConstraintDirection::TopLeft,
    //                     ));
    //                     added_directions.push(ConstraintDirection::TopLeft);
    //                 }
    //             } else {
    //                 new_valids.push(*pattern_id);
    //             }
    //         }

    //         if new_valids.len() == 0 {
    //             return false;
    //         }

    //         self.super_grid[target[0]][target[1]] = new_valids;
    //     }

    //     true
    // }
}
