#[derive(PartialEq, Eq)]
pub struct Constraint {
    pub pattern_a_id: usize,
    pub pattern_b_id: usize,
    pub direction: ConstraintDirection,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ConstraintDirection {
    Top,
    Right,
    Bottom,
    Left,
}

impl ConstraintDirection {
    pub fn reverse(&self) -> ConstraintDirection {
        match *self {
            ConstraintDirection::Top => ConstraintDirection::Bottom,
            ConstraintDirection::Right => ConstraintDirection::Left,
            ConstraintDirection::Bottom => ConstraintDirection::Top,
            ConstraintDirection::Left => ConstraintDirection::Right,
        }
    }
}
