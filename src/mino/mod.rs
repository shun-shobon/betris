mod shape;

use crate::board::Position;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mino {
    pub shape: MinoShape,
    pub rotation: MinoRotation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MinoShape {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl MinoShape {
    pub fn to_shape(&self, rotation: MinoRotation) -> &[Position] {
        match self {
            Self::I => &shape::I_SHAPE[rotation as usize],
            Self::J => &shape::J_SHAPE[rotation as usize],
            Self::L => &shape::L_SHAPE[rotation as usize],
            Self::O => &shape::O_SHAPE[rotation as usize],
            Self::S => &shape::S_SHAPE[rotation as usize],
            Self::T => &shape::T_SHAPE[rotation as usize],
            Self::Z => &shape::Z_SHAPE[rotation as usize],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MinoRotation {
    #[default]
    R0,
    R90,
    R180,
    R270,
}

impl Mino {
    pub fn new(shape: MinoShape) -> Self {
        Self {
            shape,
            rotation: MinoRotation::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MinoColor {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}
