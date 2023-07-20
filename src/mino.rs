use crate::{
    block::Block,
    field::FIELD_WIDTH,
    movement::{Direction, MoveEvent},
    position::Position,
};
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Component)]
pub struct MinoPosition(pub Position);

type MinoBlocks = [Position; 4];

#[derive(Debug, Clone, Copy, Component)]
pub struct Mino {
    blocks: MinoBlocks,
}

#[derive(Component)]
pub struct DropTimer(pub Timer);

impl Mino {
    pub fn spawn(commands: &mut Commands, mino_type: MinoType, block_size: f32) -> Entity {
        let mino = Mino {
            blocks: mino_type.blocks(),
        };

        commands
            .spawn(SpatialBundle::default())
            .insert((
                mino,
                MinoPosition(Position::new((FIELD_WIDTH - mino_type.size()) / 2, 0)),
                DropTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
            ))
            .with_children(|parent| {
                for &block_pos in mino.blocks.iter() {
                    Block::spwan_with_parent(parent, mino_type.color(), block_size, block_pos);
                }
            })
            .id()
    }
}

pub fn drop_mino_system(
    time: Res<Time>,
    mut mino_query: Query<(&mut DropTimer, &Parent), With<Mino>>,
    mut move_event_writer: EventWriter<MoveEvent>,
) {
    for (mut drop_timer, field_entity) in mino_query.iter_mut() {
        if drop_timer.0.tick(time.delta()).just_finished() {
            move_event_writer.send(MoveEvent(field_entity.get(), Direction::Down));
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MinoType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl MinoType {
    pub fn size(&self) -> i32 {
        match self {
            Self::I => 4,
            Self::J | Self::L | Self::S | Self::T | Self::Z => 3,
            Self::O => 2,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            MinoType::I => Color::rgb(0.0, 1.0, 1.0),
            MinoType::J => Color::rgb(0.0, 0.0, 1.0),
            MinoType::L => Color::rgb(1.0, 0.5, 0.0),
            MinoType::O => Color::rgb(1.0, 0.5, 0.0),
            MinoType::S => Color::rgb(0.0, 1.0, 0.0),
            MinoType::T => Color::rgb(0.5, 0.0, 1.0),
            MinoType::Z => Color::rgb(1.0, 0.0, 0.0),
        }
    }

    pub fn blocks(&self) -> MinoBlocks {
        match self {
            MinoType::I => [
                Position::new(0, 1),
                Position::new(1, 1),
                Position::new(2, 1),
                Position::new(3, 1),
            ],
            MinoType::J => [
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(1, 1),
                Position::new(2, 1),
            ],
            MinoType::L => [
                Position::new(2, 0),
                Position::new(0, 1),
                Position::new(1, 1),
                Position::new(2, 1),
            ],
            MinoType::O => [
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(0, 1),
                Position::new(1, 1),
            ],
            MinoType::S => [
                Position::new(1, 0),
                Position::new(2, 0),
                Position::new(0, 1),
                Position::new(1, 1),
            ],
            MinoType::T => [
                Position::new(1, 0),
                Position::new(0, 1),
                Position::new(1, 1),
                Position::new(2, 1),
            ],
            MinoType::Z => [
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(1, 1),
                Position::new(2, 1),
            ],
        }
    }
}
