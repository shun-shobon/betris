pub mod event;
pub mod shape;
pub mod t_spin;

use self::shape::Shape;
use crate::{
    field::{FIELD_HEIGHT, FIELD_WIDTH},
    pos,
    position::Position,
};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Component, Serialize, Deserialize)]
pub struct Mino {
    pub pos: Position,
    pub angle: Angle,
    pub shape: Shape,
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
    pub fn new(shape: Shape) -> Self {
        Self {
            pos: pos!(
                (FIELD_WIDTH - shape.width()) / 2,
                FIELD_HEIGHT - 2 - shape.spawn_y_offset(), // TODO: 20行目が埋まっている場合は21行目に出現させる
            ),
            angle: Angle::default(),
            shape,
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
