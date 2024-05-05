use crate::{
    field::{local::LocalField, Field},
    mino::{event::PlaceMinoEvent, Mino},
    movement::{Direction, MoveEvent},
    net::{PlayerState, Players},
};
use bevy::prelude::*;
use if_chain::if_chain;
use std::time::Duration;

pub const DROP_INTERVAL: Duration = Duration::from_millis(1000);
pub const SOFT_DROP_INTERVAL: Duration = Duration::from_millis(50);
pub const LOCK_DOWN_INTERVAL: Duration = Duration::from_millis(500);
pub const TARGET_CHANGE_INTERVAL: Duration = Duration::from_millis(1000);

#[derive(Component)]
pub struct DropTimer(pub Timer);

#[derive(Component)]
pub struct LockDownTimer(pub Timer);

#[derive(Component)]
pub struct TargetChangeTimer(pub Timer);

impl Default for DropTimer {
    fn default() -> Self {
        Self(Timer::new(DROP_INTERVAL, TimerMode::Repeating))
    }
}

impl Default for LockDownTimer {
    fn default() -> Self {
        Self(Timer::new(LOCK_DOWN_INTERVAL, TimerMode::Repeating))
    }
}

impl Default for TargetChangeTimer {
    fn default() -> Self {
        Self(Timer::new(TARGET_CHANGE_INTERVAL, TimerMode::Repeating))
    }
}

pub fn drop_timer_system(
    time: Res<Time>,
    mut drop_timer_query: Query<&mut DropTimer>,
    mut move_event_writer: EventWriter<MoveEvent>,
) {
    let Ok(mut drop_timer) = drop_timer_query.get_single_mut() else {
        return;
    };
    if drop_timer.0.tick(time.delta()).just_finished() {
        move_event_writer.send(MoveEvent::Move(Direction::Down));
    }
}

pub fn lock_down_timer_system(
    time: Res<Time>,
    mut field_query: Query<(&Field, &mut LockDownTimer)>,
    mino_query: Query<&Mino>,
    mut place_mino_event_writer: EventWriter<PlaceMinoEvent>,
) {
    let Ok((field, mut lock_down_timer)) = field_query.get_single_mut() else {
        return;
    };
    let Ok(mino) = mino_query.get_single() else {
        return;
    };

    if !mino.is_landed(field) {
        lock_down_timer.0.reset();
        return;
    }

    if lock_down_timer.0.tick(time.delta()).just_finished() {
        place_mino_event_writer.send(PlaceMinoEvent);
    }
}

pub fn target_change_timer_system(
    time: Res<Time>,
    players: Res<Players>,
    mut field_query: Query<&mut LocalField, With<Field>>,
    mut target_change_timer_query: Query<&mut TargetChangeTimer>,
) {
    let Ok(mut local_field) = field_query.get_single_mut() else {
        return;
    };
    let Ok(mut target_change_timer) = target_change_timer_query.get_single_mut() else {
        return;
    };

    if target_change_timer.0.tick(time.delta()).just_finished() {
        local_field.target_player_id = if_chain! {
            if let Some(target_id) = local_field.target_player_id;
            if let Some(target) = players.0
                .iter()
                .filter(|player| player.state == PlayerState::Playing)
                .skip_while(|player| player.id != target_id)
                .nth(1);
            then {
                Some(target.id)
            } else {
                players.0.iter().find(|player| player.state == PlayerState::Playing).map(|player| player.id)
            }
        };
    }
}
