use crate::position::Position;
use bevy::prelude::*;

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
    pub const COUNT: usize = 7;

    pub fn size(&self) -> i8 {
        match self {
            Self::I => 4,
            Self::J | Self::L | Self::S | Self::T | Self::Z => 3,
            Self::O => 2,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            MinoShape::I => Color::rgb(0.0, 1.0, 1.0),
            MinoShape::J => Color::rgb(0.0, 0.0, 1.0),
            MinoShape::L => Color::rgb(1.0, 0.5, 0.0),
            MinoShape::O => Color::rgb(1.0, 1.0, 0.0),
            MinoShape::S => Color::rgb(0.0, 1.0, 0.0),
            MinoShape::T => Color::rgb(0.5, 0.0, 1.0),
            MinoShape::Z => Color::rgb(1.0, 0.0, 0.0),
        }
    }

    pub fn blocks(&self) -> &[Position] {
        match self {
            MinoShape::I => I_SHAPE,
            MinoShape::J => J_SHAPE,
            MinoShape::L => L_SHAPE,
            MinoShape::O => O_SHAPE,
            MinoShape::S => S_SHAPE,
            MinoShape::T => T_SHAPE,
            MinoShape::Z => Z_SHAPE,
        }
    }
}

static I_SHAPE: &[Position] = &[
    Position::new(0, 2),
    Position::new(1, 2),
    Position::new(2, 2),
    Position::new(3, 2),
];
static J_SHAPE: &[Position] = &[
    Position::new(0, 2),
    Position::new(0, 1),
    Position::new(1, 1),
    Position::new(2, 1),
];
static L_SHAPE: &[Position] = &[
    Position::new(2, 2),
    Position::new(0, 1),
    Position::new(1, 1),
    Position::new(2, 1),
];
static O_SHAPE: &[Position] = &[
    Position::new(0, 0),
    Position::new(1, 0),
    Position::new(0, 1),
    Position::new(1, 1),
];
static S_SHAPE: &[Position] = &[
    Position::new(1, 2),
    Position::new(2, 2),
    Position::new(0, 1),
    Position::new(1, 1),
];
static T_SHAPE: &[Position] = &[
    Position::new(1, 2),
    Position::new(0, 1),
    Position::new(1, 1),
    Position::new(2, 1),
];
static Z_SHAPE: &[Position] = &[
    Position::new(0, 2),
    Position::new(1, 2),
    Position::new(1, 1),
    Position::new(2, 1),
];
