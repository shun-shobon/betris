pub mod event;
pub mod shape;
pub mod t_spin;

use self::shape::Shape;
use crate::{
    field::{Field, FIELD_HEIGHT, FIELD_WIDTH},
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
    pub fn new(shape: Shape, field: &Field) -> Result<Self, ()> {
        let pos = (0..=2)
            .rev()
            .map(|offset_y| {
                pos!(
                    (FIELD_WIDTH - shape.width()) / 2,
                    FIELD_HEIGHT - offset_y - shape.offset_y(),
                )
            })
            .find(|&pos| field.blocks.can_place_mino(pos, shape, Angle::default()));

        if let Some(pos) = pos {
            Ok(Self {
                pos,
                angle: Angle::default(),
                shape,
            })
        } else {
            Err(())
        }
    }

    pub fn spawn(self, commands: &mut Commands) -> Entity {
        commands.spawn((SpatialBundle::default(), self)).id()
    }

    pub fn is_landed(&self, field: &Field) -> bool {
        !field
            .blocks
            .can_place_mino(self.pos + pos!(0, -1), self.shape, self.angle)
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
