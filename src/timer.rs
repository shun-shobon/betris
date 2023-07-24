use crate::{
    field::{local::LocalField, Field},
    movement::{Direction, MoveEvent},
    net::{LocalPlaceMinoEvent, Players},
};
use bevy::prelude::*;
use std::time::Duration;

pub const DROP_INTERVAL: Duration = Duration::from_millis(1000);
pub const SOFT_DROP_INTERVAL: Duration = Duration::from_millis(50);
pub const LOCK_DOWN_INTERVAL: Duration = Duration::from_millis(500);
pub const TARGET_CHANGE_INTERVAL: Duration = Duration::from_millis(1000);

pub fn create_drop_timer() -> Timer {
    Timer::new(DROP_INTERVAL, TimerMode::Repeating)
}

pub fn create_lock_down_timer() -> Timer {
    let mut timer = Timer::new(LOCK_DOWN_INTERVAL, TimerMode::Once);
    timer.pause();

    timer
}

pub fn create_target_change_timer() -> Timer {
    Timer::new(TARGET_CHANGE_INTERVAL, TimerMode::Repeating)
}

pub fn timer_system(
    time: Res<Time>,
    players: Res<Players>,
    mut field_query: Query<&mut LocalField, With<Field>>,
    mut move_event_writer: EventWriter<MoveEvent>,
    mut local_place_mino_event_writer: EventWriter<LocalPlaceMinoEvent>,
) {
    let Ok(mut local_field ) = field_query.get_single_mut() else { return; };

    if local_field.drop_timer.tick(time.delta()).just_finished() {
        move_event_writer.send(MoveEvent::Move(Direction::Down));
    }

    if local_field
        .lock_down_timer
        .tick(time.delta())
        .just_finished()
    {
        local_place_mino_event_writer.send(LocalPlaceMinoEvent);
    }

    if local_field
        .target_change_timer
        .tick(time.delta())
        .just_finished()
    {
        let next_target_player_id = if let Some(target_player_id) = local_field.target_player_id {
            let target_player_idx = players
                .0
                .iter()
                .position(|player_id| *player_id == target_player_id)
                .unwrap();
            let next_target_player_idx = (target_player_idx + 1) % players.0.len();
            Some(players.0[next_target_player_idx])
        } else {
            players.0.first().copied()
        };

        local_field.target_player_id = next_target_player_id;
    }
}
