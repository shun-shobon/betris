use crate::{
    field::{Field, LocalField},
    mino::event::PlaceMinoEvent,
    movement::{Direction, MoveEvent},
};
use bevy::prelude::*;
use std::time::Duration;

pub const DROP_INTERVAL: Duration = Duration::from_millis(1000);
pub const SOFT_DROP_INTERVAL: Duration = Duration::from_millis(50);
pub const LOCK_DOWN_INTERVAL: Duration = Duration::from_millis(500);

pub fn timer_system(
    time: Res<Time>,
    mut timer_query: Query<(Entity, &mut LocalField), With<Field>>,
    mut move_event_writer: EventWriter<MoveEvent>,
    mut place_mino_event_writer: EventWriter<PlaceMinoEvent>,
) {
    for (field_entity, mut local_field) in timer_query.iter_mut() {
        if local_field.drop_timer.tick(time.delta()).just_finished() {
            move_event_writer.send(MoveEvent::Move(Direction::Down));
        }
        if local_field
            .lock_down_timer
            .tick(time.delta())
            .just_finished()
        {
            place_mino_event_writer.send(PlaceMinoEvent(field_entity));
        }
    }
}
