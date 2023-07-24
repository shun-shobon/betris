use bevy::prelude::*;

use crate::{
    mino::{shape::MinoShape, Mino},
    position::Position,
};

pub const BLOCK_SIZE: f32 = 40.0;
pub const BLOCK_INSET: f32 = 1.0;

use super::Field;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Component)]
pub enum FieldBlock {
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

impl FieldBlock {
    #[must_use]
    pub fn color(&self) -> Color {
        use FieldBlock::{I, J, L, O, S, T, Z};

        match self {
            I => Color::rgb(0.0, 1.0, 1.0),
            J => Color::rgb(0.0, 0.0, 1.0),
            L => Color::rgb(1.0, 0.5, 0.0),
            O => Color::rgb(1.0, 1.0, 0.0),
            S => Color::rgb(0.0, 1.0, 0.0),
            T => Color::rgb(0.5, 0.0, 1.0),
            Z => Color::rgb(1.0, 0.0, 0.0),
            _ => unreachable!(),
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self == &Self::Empty
    }
}

impl From<MinoShape> for FieldBlock {
    fn from(shape: MinoShape) -> Self {
        match shape {
            MinoShape::I => FieldBlock::I,
            MinoShape::J => FieldBlock::J,
            MinoShape::L => FieldBlock::L,
            MinoShape::O => FieldBlock::O,
            MinoShape::S => FieldBlock::S,
            MinoShape::T => FieldBlock::T,
            MinoShape::Z => FieldBlock::Z,
        }
    }
}

#[allow(
    clippy::needless_pass_by_value,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
)]
pub fn field_block_system(
    mut commands: Commands,
    field_block_query: Query<Entity, With<FieldBlock>>,
    field_query: Query<(Entity, &Field)>,
    mino_query: Query<(Entity, &Mino)>,
) {
    for block_entity in field_block_query.iter() {
        commands.entity(block_entity).despawn_recursive();
    }

    for (field_entity, field) in field_query.iter() {
        let field_block_bundles = field
            .lines
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, block)| (Position::new(x as i8, y as i8), block))
            })
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

fn create_field_block_bundle(pos: Position, block: FieldBlock) -> (SpriteBundle, FieldBlock) {
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
