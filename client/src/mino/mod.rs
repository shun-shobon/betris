pub mod event;
pub mod shape;

use self::shape::MinoShape;
use crate::{
    field::{FIELD_HEIGHT, FIELD_WIDTH},
    position::Position,
};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Component, Serialize, Deserialize)]
pub struct Mino {
    pub pos: Position,
    pub angle: Angle,
    pub shape: MinoShape,
    pub t_spin: TSpin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TSpin {
    #[default]
    None,
    Mini,
    Full,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Angle {
    #[default]
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

impl Mino {
    pub fn new(shape: MinoShape) -> Self {
        Self {
            pos: Position::new(
                (FIELD_WIDTH - shape.size()) / 2,
                FIELD_HEIGHT - 2, // TODO: 20行目が埋まっている場合は21行目に出現させる
            ),
            angle: Angle::default(),
            shape,
            t_spin: TSpin::default(),
        }
    }

    pub fn spawn(self, commands: &mut Commands) -> Entity {
        commands.spawn((SpatialBundle::default(), self)).id()
    }
}

impl From<Angle> for usize {
    fn from(angle: Angle) -> Self {
        match angle {
            Angle::Deg0 => 0,
            Angle::Deg90 => 1,
            Angle::Deg180 => 2,
            Angle::Deg270 => 3,
        }
    }
}
