use bevy::prelude::*;

use crate::position::Position;

use super::MinoBlocks;

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
    pub fn size(&self) -> i32 {
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
            MinoShape::O => Color::rgb(1.0, 0.5, 0.0),
            MinoShape::S => Color::rgb(0.0, 1.0, 0.0),
            MinoShape::T => Color::rgb(0.5, 0.0, 1.0),
            MinoShape::Z => Color::rgb(1.0, 0.0, 0.0),
        }
    }

    pub fn blocks(&self) -> MinoBlocks {
        match self {
            MinoShape::I => [
                Position::new(0, 1),
                Position::new(1, 1),
                Position::new(2, 1),
                Position::new(3, 1),
            ],
            MinoShape::J => [
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(1, 1),
                Position::new(2, 1),
            ],
            MinoShape::L => [
                Position::new(2, 0),
                Position::new(0, 1),
                Position::new(1, 1),
                Position::new(2, 1),
            ],
            MinoShape::O => [
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(0, 1),
                Position::new(1, 1),
            ],
            MinoShape::S => [
                Position::new(1, 0),
                Position::new(2, 0),
                Position::new(0, 1),
                Position::new(1, 1),
            ],
            MinoShape::T => [
                Position::new(1, 0),
                Position::new(0, 1),
                Position::new(1, 1),
                Position::new(2, 1),
            ],
            MinoShape::Z => [
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(1, 1),
                Position::new(2, 1),
            ],
        }
    }
}
