use std::time::Duration;

use bevy::prelude::*;

use crate::{
    field::local::HoldEvent,
    movement::{Direction, MoveEvent},
};

const MOVE_REPLEAT_DELAY: Duration = Duration::from_millis(300);
const MOVE_REPLEAT_INTERVAL: Duration = Duration::from_millis(30);

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
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut repeat_timer: ResMut<KeyboardRepeatTimer>,
    mut move_event_writer: EventWriter<MoveEvent>,
    mut hold_event_writer: EventWriter<HoldEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        repeat_timer.0.set_duration(MOVE_REPLEAT_DELAY);
        repeat_timer.0.reset();

        move_event_writer.send(MoveEvent::Move(Direction::Left));
    }
    if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        repeat_timer.0.set_duration(MOVE_REPLEAT_DELAY);
        repeat_timer.0.reset();

        move_event_writer.send(MoveEvent::Move(Direction::Right));
    }

    if !repeat_timer.0.finished() {
        repeat_timer.0.tick(time.delta());
    } else {
        repeat_timer.0.set_duration(MOVE_REPLEAT_INTERVAL);
        repeat_timer.0.reset();

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            move_event_writer.send(MoveEvent::Move(Direction::Left));
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            move_event_writer.send(MoveEvent::Move(Direction::Right));
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyZ) {
        move_event_writer.send(MoveEvent::Rotate(Direction::Left));
    } else if keyboard_input.just_pressed(KeyCode::KeyX) {
        move_event_writer.send(MoveEvent::Rotate(Direction::Right));
    }

    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        move_event_writer.send(MoveEvent::HardDrop);
    }
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        move_event_writer.send(MoveEvent::StartSoftDrop);
    } else if keyboard_input.just_released(KeyCode::ArrowDown) {
        move_event_writer.send(MoveEvent::StopSoftDrop);
    }

    if keyboard_input.just_pressed(KeyCode::ShiftLeft) {
        hold_event_writer.send(HoldEvent);
    }
}
