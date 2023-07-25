use bevy::prelude::*;

use crate::{
    mino::{shape::Shape, Mino},
    position::Position,
};

pub const BLOCK_SIZE: f32 = 40.0;
pub const BLOCK_INSET: f32 = 1.0;

use super::Field;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Component)]
pub enum Block {
    #[default]
    Empty,
    Garbage,
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl Block {
    pub fn color(&self) -> Color {
        match self {
            Block::Empty => Color::NONE,
            Block::I => Color::rgb(0.0, 1.0, 1.0),
            Block::J => Color::rgb(0.0, 0.0, 1.0),
            Block::L => Color::rgb(1.0, 0.5, 0.0),
            Block::O => Color::rgb(1.0, 1.0, 0.0),
            Block::S => Color::rgb(0.0, 1.0, 0.0),
            Block::T => Color::rgb(0.5, 0.0, 1.0),
            Block::Z => Color::rgb(1.0, 0.0, 0.0),
            Block::Garbage => Color::rgb(0.5, 0.5, 0.5),
        }
    }

    pub fn is_empty(&self) -> bool {
        self == &Self::Empty
    }

    pub fn is_filled(&self) -> bool {
        !self.is_empty()
    }
}

impl From<Shape> for Block {
    fn from(shape: Shape) -> Self {
        match shape {
            Shape::I => Block::I,
            Shape::J => Block::J,
            Shape::L => Block::L,
            Shape::O => Block::O,
            Shape::S => Block::S,
            Shape::T => Block::T,
            Shape::Z => Block::Z,
        }
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub fn field_block_system(
    mut commands: Commands,
    field_block_query: Query<Entity, With<Block>>,
    field_query: Query<(Entity, &Field)>,
    mino_query: Query<(Entity, &Mino)>,
) {
    for block_entity in field_block_query.iter() {
        commands.entity(block_entity).despawn_recursive();
    }

    for (field_entity, field) in field_query.iter() {
        let field_block_bundles = field
            .blocks
            .indexed_iter()
            .filter(|(_, block)| !block.is_empty())
            .map(move |(pos, &block)| create_field_block_bundle(pos, block))
            .collect::<Vec<_>>();

        commands.entity(field_entity).with_children(|parent| {
            for bundle in field_block_bundles {
                parent.spawn(bundle);
            }
        });
    }

    for (mino_entity, mino) in mino_query.iter() {
        let mino_block_bundles = mino
            .shape
            .blocks(mino.angle)
            .iter()
            .map(|&pos| pos + mino.pos)
            .map(|pos| create_field_block_bundle(pos, mino.shape.into()))
            .collect::<Vec<_>>();

        commands.entity(mino_entity).with_children(|parent| {
            for bundle in mino_block_bundles {
                parent.spawn(bundle);
            }
        });
    }
}

fn create_field_block_bundle(pos: Position, block: Block) -> (SpriteBundle, Block) {
    let bundle = SpriteBundle {
        transform: Transform::from_translation(pos.translation()),
        sprite: Sprite {
            color: block.color(),
            custom_size: Some(Vec2::new(
                BLOCK_SIZE - BLOCK_INSET,
                BLOCK_SIZE - BLOCK_INSET,
            )),
            ..default()
        },
        ..default()
    };

    (bundle, block)
}
