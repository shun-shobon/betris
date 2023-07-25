use crate::{
    field::{local::LocalField, Field},
    mino::{
        event::{PlaceMinoEvent, SpawnMinoEvent},
        Mino,
    },
    movement::{Direction, MoveEvent},
    net::Players,
};
use bevy::prelude::*;
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
        let mut timer = Timer::new(LOCK_DOWN_INTERVAL, TimerMode::Once);
        timer.pause();

        Self(timer)
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
    let Ok(mut drop_timer) = drop_timer_query.get_single_mut() else { return; };
    if drop_timer.0.tick(time.delta()).just_finished() {
        move_event_writer.send(MoveEvent::Move(Direction::Down));
    }
}

pub fn lock_down_timer_system(
    mut commands: Commands,
    time: Res<Time>,
    mut lock_down_timer_query: Query<&mut LockDownTimer>,
    mino_query: Query<(Entity, &Mino)>,
    mut place_mino_event_writer: EventWriter<PlaceMinoEvent>,
    mut spwan_mino_event_writer: EventWriter<SpawnMinoEvent>,
) {
    let Ok(mut lock_down_timer) = lock_down_timer_query.get_single_mut() else { return; };
    if lock_down_timer.0.tick(time.delta()).just_finished() {
        let Ok((mino_entity, mino)) = mino_query.get_single() else { return; };
        commands.entity(mino_entity).despawn_recursive();
        place_mino_event_writer.send(PlaceMinoEvent(*mino));
        spwan_mino_event_writer.send(SpawnMinoEvent);
    }
}

pub fn target_change_timer_system(
    time: Res<Time>,
    players: Res<Players>,
    mut field_query: Query<&mut LocalField, With<Field>>,
    mut target_change_timer_query: Query<&mut TargetChangeTimer>,
) {
    let Ok(mut local_field) = field_query.get_single_mut() else { return; };
    let Ok(mut target_change_timer) = target_change_timer_query.get_single_mut() else { return; };

    if target_change_timer.0.tick(time.delta()).just_finished() {
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
