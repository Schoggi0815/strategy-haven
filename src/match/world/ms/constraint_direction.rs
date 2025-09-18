pub const LAST_CONSTRAINT_DIRECTION: ConstraintDirection = ConstraintDirection::BottomRight;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum ConstraintDirection {
    Top,
    Right,
    Bottom,
    Left,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl ConstraintDirection {
    pub fn reverse(&self) -> ConstraintDirection {
        match *self {
            ConstraintDirection::Top => ConstraintDirection::Bottom,
            ConstraintDirection::Right => ConstraintDirection::Left,
            ConstraintDirection::Bottom => ConstraintDirection::Top,
            ConstraintDirection::Left => ConstraintDirection::Right,
            ConstraintDirection::TopLeft => ConstraintDirection::BottomRight,
            ConstraintDirection::TopRight => ConstraintDirection::BottomLeft,
            ConstraintDirection::BottomLeft => ConstraintDirection::TopRight,
            ConstraintDirection::BottomRight => ConstraintDirection::TopLeft,
        }
    }

    pub fn flip_x(&self) -> ConstraintDirection {
        match *self {
            ConstraintDirection::Top => ConstraintDirection::Top,
            ConstraintDirection::Right => ConstraintDirection::Left,
            ConstraintDirection::Bottom => ConstraintDirection::Bottom,
            ConstraintDirection::Left => ConstraintDirection::Right,
            ConstraintDirection::TopLeft => ConstraintDirection::TopRight,
            ConstraintDirection::TopRight => ConstraintDirection::TopLeft,
            ConstraintDirection::BottomLeft => ConstraintDirection::BottomRight,
            ConstraintDirection::BottomRight => ConstraintDirection::BottomLeft,
        }
    }

    pub fn flip_y(&self) -> ConstraintDirection {
        match *self {
            ConstraintDirection::Top => ConstraintDirection::Bottom,
            ConstraintDirection::Right => ConstraintDirection::Right,
            ConstraintDirection::Bottom => ConstraintDirection::Top,
            ConstraintDirection::Left => ConstraintDirection::Left,
            ConstraintDirection::TopLeft => ConstraintDirection::BottomLeft,
            ConstraintDirection::TopRight => ConstraintDirection::BottomRight,
            ConstraintDirection::BottomLeft => ConstraintDirection::TopLeft,
            ConstraintDirection::BottomRight => ConstraintDirection::TopRight,
        }
    }
}

impl From<usize> for ConstraintDirection {
    fn from(value: usize) -> Self {
        match value {
            0 => ConstraintDirection::Top,
            1 => ConstraintDirection::Right,
            2 => ConstraintDirection::Bottom,
            3 => ConstraintDirection::Left,
            4 => ConstraintDirection::TopLeft,
            5 => ConstraintDirection::TopRight,
            6 => ConstraintDirection::BottomLeft,
            7 => ConstraintDirection::BottomRight,
            _ => ConstraintDirection::Bottom,
        }
    }
}
