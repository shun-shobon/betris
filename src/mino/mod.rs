pub mod shape;
pub mod timer;

use self::{
    shape::MinoShape,
    timer::{DropTimer, LockDownTimer},
};
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

impl Mino {
    pub fn spawn(commands: &mut Commands, mino_type: MinoShape, block_size: f32) -> Entity {
        let mino = Mino {
            blocks: mino_type.blocks(),
        };

        commands
            .spawn(SpatialBundle::default())
            .insert((
                mino,
                MinoPosition(Position::new((FIELD_WIDTH - mino_type.size()) / 2, 0)),
                DropTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
                LockDownTimer(Timer::from_seconds(0.5, TimerMode::Once)),
            ))
            .with_children(|parent| {
                for &block_pos in mino.blocks.iter() {
                    Block::spwan_with_parent(parent, mino_type.color(), block_size, block_pos);
                }
            })
            .id()
    }
}

pub fn mino_timer_system(
    time: Res<Time>,
    mut mino_query: Query<(&mut DropTimer, &mut LockDownTimer, &Parent), With<Mino>>,
    mut move_event_writer: EventWriter<MoveEvent>,
) {
    for (mut drop_timer, mut lock_down_timer, field_entity) in mino_query.iter_mut() {
        if drop_timer.0.tick(time.delta()).just_finished() {
            move_event_writer.send(MoveEvent(field_entity.get(), Direction::Down));
        }
        if lock_down_timer.0.tick(time.delta()).just_finished() {
            todo!()
        }
    }
}
