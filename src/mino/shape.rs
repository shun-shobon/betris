use super::Angle;
use crate::{field::block::Block, pos, position::Position};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Shape {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl Shape {
    pub const COUNT: usize = 7;

    pub const fn width(&self) -> i8 {
        match self {
            Self::I => 4,
            Self::J | Self::L | Self::S | Self::T | Self::Z => 3,
            Self::O => 2,
        }
    }
    pub fn spawn_y_offset(&self) -> i8 {
        match self {
            Self::O => 0,
            Self::I => 2,
            Self::J | Self::L | Self::S | Self::T | Self::Z => 1,
        }
    }

    pub fn color(&self) -> Color {
        Block::from(*self).color()
    }

    pub fn blocks(&self, angle: Angle) -> &[Position] {
        let angle_idx: usize = angle.into();

        match self {
            Shape::I => &I_SHAPES[angle_idx],
            Shape::J => &J_SHAPES[angle_idx],
            Shape::L => &L_SHAPES[angle_idx],
            Shape::O => &O_SHAPES[angle_idx],
            Shape::S => &S_SHAPES[angle_idx],
            Shape::T => &T_SHAPES[angle_idx],
            Shape::Z => &Z_SHAPES[angle_idx],
        }
    }
}

macro_rules! define_shape {
    ($shape:expr; $(($x:expr, $y:expr)),*) => {
        [
            [$(pos!($x, $y)),*],
            [$(pos!($y, 1 - ($x - ($shape.width() - 2)))),*],
            [$(pos!(($shape.width() - 1) - $x, ($shape.width() - 1) - $y)),*],
            [$(pos!(1 - ($y - ($shape.width() - 2)), $x)),*],
        ]
    };
}

type MinoShapes = [[Position; 4]; 4];

static I_SHAPES: MinoShapes = define_shape!(Shape::I; (0, 2), (1, 2), (2, 2), (3, 2));
static J_SHAPES: MinoShapes = define_shape!(Shape::J; (0, 2), (0, 1), (1, 1), (2, 1));
static L_SHAPES: MinoShapes = define_shape!(Shape::L; (2, 2), (0, 1), (1, 1), (2, 1));
static O_SHAPES: MinoShapes = define_shape!(Shape::O; (0, 1), (1, 1), (0, 0), (1, 0));
static S_SHAPES: MinoShapes = define_shape!(Shape::S; (1, 2), (2, 2), (0, 1), (1, 1));
static T_SHAPES: MinoShapes = define_shape!(Shape::T; (1, 2), (0, 1), (1, 1), (2, 1));
static Z_SHAPES: MinoShapes = define_shape!(Shape::Z; (0, 2), (1, 2), (1, 1), (2, 1));
