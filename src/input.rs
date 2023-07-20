use std::time::Duration;

use bevy::prelude::*;

use crate::{
    field::Field,
    movement::{Direction, MoveEvent},
};

const MOVE_REPLEAT_DELAY: Duration = Duration::from_millis(300);
const MOVE_REPLEAT_INTERVAL: Duration = Duration::from_millis(50);

#[derive(Resource)]
pub struct KeyboardRepeatTimer(Timer);

impl Default for KeyboardRepeatTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(
            MOVE_REPLEAT_DELAY.as_secs_f32(),
            TimerMode::Repeating,
        ))
    }
}

pub fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut repeat_timer: ResMut<KeyboardRepeatTimer>,
    mut move_event: EventWriter<MoveEvent>,
    field_query: Query<(Entity, &Field)>,
) {
    let Some(field_entity) = field_query.iter().find(|(_, field)| field.id == 0).map(|(entity, _)| entity) else { return; };

    if keyboard_input.just_pressed(KeyCode::Left) {
        repeat_timer.0.set_duration(MOVE_REPLEAT_DELAY);
        repeat_timer.0.reset();

        move_event.send(MoveEvent(field_entity, Direction::Left));
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        repeat_timer.0.set_duration(MOVE_REPLEAT_DELAY);
        repeat_timer.0.reset();

        move_event.send(MoveEvent(field_entity, Direction::Right));
    }

    if !repeat_timer.0.finished() {
        repeat_timer.0.tick(time.delta());
    } else {
        repeat_timer.0.set_duration(MOVE_REPLEAT_INTERVAL);
        repeat_timer.0.reset();

        if keyboard_input.pressed(KeyCode::Left) {
            move_event.send(MoveEvent(field_entity, Direction::Left));
        }
        if keyboard_input.pressed(KeyCode::Right) {
            move_event.send(MoveEvent(field_entity, Direction::Right));
        }
    }
}
