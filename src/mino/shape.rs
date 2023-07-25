use super::Angle;
use crate::{field::block::Block, position::Position};
use bevy::prelude::*;
use once_cell::sync::Lazy;
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

    pub fn size(&self) -> i8 {
        match self {
            Self::I => 4,
            Self::J | Self::L | Self::S | Self::T | Self::Z => 3,
            Self::O => 2,
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

type MinoShapes = [Vec<Position>; 4];

static I_SHAPES: Lazy<MinoShapes> = Lazy::new(|| {
    define_mino_shapes(
        &[
            Position::new(0, 2),
            Position::new(1, 2),
            Position::new(2, 2),
            Position::new(3, 2),
        ],
        Shape::I,
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
        Shape::J,
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
        Shape::L,
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
        Shape::O,
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
        Shape::S,
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
        Shape::T,
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
        Shape::Z,
    )
});

fn define_mino_shapes(deg0_pos: &[Position], shape: Shape) -> [Vec<Position>; 4] {
    let deg90_pos = deg0_pos
        .iter()
        .map(|pos| Position::new(pos.y, 1 - (pos.x - (shape.size() - 2))))
        .collect::<Vec<_>>();
    let deg180_pos = deg0_pos
        .iter()
        .map(|pos| Position::new((shape.size() - 1) - pos.x, (shape.size() - 1) - pos.y))
        .collect::<Vec<_>>();
    let deg270_pos = deg0_pos
        .iter()
        .map(|pos| Position::new(1 - (pos.y - (shape.size() - 2)), pos.x))
        .collect::<Vec<_>>();

    [deg0_pos.to_vec(), deg90_pos, deg180_pos, deg270_pos]
}
