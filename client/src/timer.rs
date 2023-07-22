use crate::{
    field::{Field, LocalField},
    movement::{Direction, MoveEvent},
    net::LocalPlaceMinoEvent,
};
use bevy::prelude::*;
use std::time::Duration;

pub const DROP_INTERVAL: Duration = Duration::from_millis(1000);
pub const SOFT_DROP_INTERVAL: Duration = Duration::from_millis(50);
pub const LOCK_DOWN_INTERVAL: Duration = Duration::from_millis(500);

pub fn timer_system(
    time: Res<Time>,
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
}
