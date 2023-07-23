use crate::{
    block::BLOCK_INSET,
    mino::shape::MinoShape,
    net::PlayerId,
    position::Position,
    random::RandomBag,
    timer::{DROP_INTERVAL, LOCK_DOWN_INTERVAL},
};
use bevy::prelude::*;

pub const FIELD_WIDTH: i8 = 10;
pub const FIELD_HEIGHT: i8 = 20;
// この値よりもブロックがせり上がった場合はゲームオーバー
pub const FIELD_MAX_HEIGHT: i8 = FIELD_HEIGHT + 20;

const FIELD_GRID_WIDTH: f32 = 1.;

type Lines = [[FieldBlock; FIELD_WIDTH as usize]; FIELD_MAX_HEIGHT as usize];

#[derive(Component)]
pub struct Field {
    pub player_id: PlayerId,
    pub block_size: f32,
    pub lines: Lines,
}

#[derive(Component)]
pub struct LocalField {
    pub random_bag: RandomBag,
    pub drop_timer: Timer,
    pub lock_down_timer: Timer,
}

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

impl Field {
    pub fn new(player_id: PlayerId, block_size: f32) -> Self {
        let lines = [[FieldBlock::default(); FIELD_WIDTH as usize]; FIELD_MAX_HEIGHT as usize];

        Self {
            player_id,
            block_size,
            lines,
        }
    }

    pub fn spawn(self, commands: &mut Commands, is_local_field: bool, translation: Vec3) -> Entity {
        let block_size = self.block_size;

        let mut field_commands = commands.spawn((
            SpatialBundle::from_transform(Transform::from_translation(translation)),
            self,
        ));

        if is_local_field {
            field_commands
                .insert(LocalField::default())
                .with_children(|parent| spawn_grid(parent, block_size))
                .id()
        } else {
            field_commands
                .with_children(|parent| spawn_grid(parent, block_size))
                .id()
        }
    }
}

impl FieldBlock {
    pub fn color(&self) -> Color {
        use FieldBlock::*;

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

pub fn field_block_system(
    mut commands: Commands,
    field_query: Query<(Entity, &Field)>,
    field_block_query: Query<Entity, With<FieldBlock>>,
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
                    .filter(|(_, &block)| block != FieldBlock::Empty)
                    .map(move |(x, &block)| {
                        let pos = Position::new(x as i8, y as i8);

                        let bundle = SpriteBundle {
                            transform: Transform::from_translation(
                                pos.translation(field.block_size),
                            ),
                            sprite: Sprite {
                                color: block.color(),
                                custom_size: Some(Vec2::new(
                                    field.block_size - BLOCK_INSET,
                                    field.block_size - BLOCK_INSET,
                                )),
                                ..default()
                            },
                            ..default()
                        };

                        (bundle, block)
                    })
            })
            .collect::<Vec<_>>();

        commands.entity(field_entity).with_children(|parent| {
            for bundle in field_block_bundles {
                parent.spawn(bundle);
            }
        });
    }
}

impl Default for LocalField {
    fn default() -> Self {
        let mut lock_down_timer = Timer::new(LOCK_DOWN_INTERVAL, TimerMode::Once);
        lock_down_timer.pause();

        Self {
            random_bag: RandomBag::new(),
            drop_timer: Timer::new(DROP_INTERVAL, TimerMode::Repeating),
            lock_down_timer,
        }
    }
}

fn spawn_grid(parent: &mut ChildBuilder, block_size: f32) {
    let width = FIELD_WIDTH as f32 * block_size;
    let height = FIELD_HEIGHT as f32 * block_size;

    for y in 0..=FIELD_HEIGHT {
        parent.spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., -(y as f32 - FIELD_HEIGHT as f32 / 2.) * block_size, 0.),
                ..default()
            },
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(width, FIELD_GRID_WIDTH)),
                ..default()
            },
            ..default()
        });
    }

    for x in 0..=FIELD_WIDTH {
        parent.spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new((x as f32 - FIELD_WIDTH as f32 / 2.) * block_size, 0., 0.),
                ..default()
            },
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(FIELD_GRID_WIDTH, height)),
                ..default()
            },
            ..default()
        });
    }
}
