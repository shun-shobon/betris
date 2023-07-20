use crate::{
    field::Field,
    mino::event::PlaceMinoEvent,
    movement::{Direction, MoveEvent},
};
use bevy::prelude::*;
use std::time::Duration;

pub const DROP_INTERVAL: Duration = Duration::from_millis(1000);
pub const SOFT_DROP_INTERVAL: Duration = Duration::from_millis(50);
pub const LOCK_DOWN_INTERVAL: Duration = Duration::from_millis(500);

#[derive(Component)]
pub struct DropTimer(pub Timer);

#[derive(Component)]
pub struct LockDownTimer(pub Timer);

impl Default for DropTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(
            DROP_INTERVAL.as_secs_f32(),
            TimerMode::Repeating,
        ))
    }
}

impl Default for LockDownTimer {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(LOCK_DOWN_INTERVAL.as_secs_f32(), TimerMode::Once);
        timer.pause();
        Self(timer)
    }
}

pub fn timer_system(
    time: Res<Time>,
    mut timer_query: Query<(Entity, &mut DropTimer, &mut LockDownTimer), With<Field>>,
    mut move_event_writer: EventWriter<MoveEvent>,
    mut place_mino_event_writer: EventWriter<PlaceMinoEvent>,
) {
    for (field_entity, mut drop_timer, mut lock_down_timer) in timer_query.iter_mut() {
        if drop_timer.0.tick(time.delta()).just_finished() {
            move_event_writer.send(MoveEvent::Move(field_entity, Direction::Down));
        }
        if lock_down_timer.0.tick(time.delta()).just_finished() {
            place_mino_event_writer.send(PlaceMinoEvent(field_entity));
        }
    }
}
