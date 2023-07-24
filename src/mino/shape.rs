use super::Angle;
use crate::position::Position;
use bevy::prelude::*;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

    pub fn blocks(&self, angle: Angle) -> &[Position] {
        let angle_idx: usize = angle.into();

        match self {
            MinoShape::I => &I_SHAPES[angle_idx],
            MinoShape::J => &J_SHAPES[angle_idx],
            MinoShape::L => &L_SHAPES[angle_idx],
            MinoShape::O => &O_SHAPES[angle_idx],
            MinoShape::S => &S_SHAPES[angle_idx],
            MinoShape::T => &T_SHAPES[angle_idx],
            MinoShape::Z => &Z_SHAPES[angle_idx],
        }
    }
}

type MinoShapes = [Vec<Position>; 4];

static I_SHAPES: Lazy<MinoShapes> = Lazy::new(|| {
    define_mino_shapes(
        &[
            Position::new(0, 2),
            Position::new(1, 2),
            Position::new(2, 2),
            Position::new(3, 2),
        ],
        4,
    )
});
static J_SHAPES: Lazy<MinoShapes> = Lazy::new(|| {
    define_mino_shapes(
        &[
            Position::new(0, 2),
            Position::new(0, 1),
            Position::new(1, 1),
            Position::new(2, 1),
        ],
        3,
    )
});
static L_SHAPES: Lazy<MinoShapes> = Lazy::new(|| {
    define_mino_shapes(
        &[
            Position::new(2, 2),
            Position::new(0, 1),
            Position::new(1, 1),
            Position::new(2, 1),
        ],
        3,
    )
});
static O_SHAPES: Lazy<MinoShapes> = Lazy::new(|| {
    define_mino_shapes(
        &[
            Position::new(0, 0),
            Position::new(1, 0),
            Position::new(0, 1),
            Position::new(1, 1),
        ],
        2,
    )
});
static S_SHAPES: Lazy<MinoShapes> = Lazy::new(|| {
    define_mino_shapes(
        &[
            Position::new(1, 2),
            Position::new(2, 2),
            Position::new(0, 1),
            Position::new(1, 1),
        ],
        3,
    )
});
static T_SHAPES: Lazy<MinoShapes> = Lazy::new(|| {
    define_mino_shapes(
        &[
            Position::new(1, 2),
            Position::new(0, 1),
            Position::new(1, 1),
            Position::new(2, 1),
        ],
        3,
    )
});
static Z_SHAPES: Lazy<MinoShapes> = Lazy::new(|| {
    define_mino_shapes(
        &[
            Position::new(0, 2),
            Position::new(1, 2),
            Position::new(1, 1),
            Position::new(2, 1),
        ],
        3,
    )
});

fn define_mino_shapes(deg0_pos: &[Position], size: i8) -> [Vec<Position>; 4] {
    let deg90_pos = deg0_pos
        .iter()
        .map(|pos| Position::new(pos.y, 1 - (pos.x - (size - 2))))
        .collect::<Vec<_>>();
    let deg180_pos = deg0_pos
        .iter()
        .map(|pos| Position::new((size - 1) - pos.x, (size - 1) - pos.y))
        .collect::<Vec<_>>();
    let deg270_pos = deg0_pos
        .iter()
        .map(|pos| Position::new(1 - (pos.y - (size - 2)), pos.x))
        .collect::<Vec<_>>();

    [deg0_pos.to_vec(), deg90_pos, deg180_pos, deg270_pos]
}
