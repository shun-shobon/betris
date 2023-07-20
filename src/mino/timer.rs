use std::time::Duration;

use bevy::prelude::*;

use super::Mino;
use crate::movement::{Direction, MoveEvent};

const DROP_INTERVAL: Duration = Duration::from_millis(1000);
const LOCK_DOWN_INTERVAL: Duration = Duration::from_millis(500);

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
        let mut timer = Timer::from_seconds(LOCK_DOWN_INTERVAL.as_secs_f32(), TimerMode::Repeating);
        timer.pause();
        Self(timer)
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
