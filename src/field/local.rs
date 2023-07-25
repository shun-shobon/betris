use super::{
    block::BLOCK_SIZE,
    random::RandomBag,
    timer::{DropTimer, LockDownTimer, TargetChangeTimer},
    FIELD_HEIGHT, FIELD_WIDTH,
};
use crate::{mino::t_spin::TSpin, net::PlayerId};
use bevy::prelude::*;
use std::collections::VecDeque;

static GARBAGE_LINE_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);
static GARBAGE_LINE_WIDTH: f32 = 20.0;
static GARBAGE_LINE_INSET: f32 = 4.0;
static GARBAGE_LINE_START_X: f32 =
    -(BLOCK_SIZE * FIELD_WIDTH as f32) / 2.0 - GARBAGE_LINE_WIDTH / 2.0 - GARBAGE_LINE_INSET;
static GARBAGE_LINE_START_Y: f32 = -(BLOCK_SIZE * FIELD_HEIGHT as f32) / 2.0;

#[derive(Debug, Event)]
pub struct ReceiveGarbageEvent(pub i8);

#[derive(Component)]
pub struct GarbageLine;

#[derive(Component)]
pub struct LocalField {
    pub can_back_to_back: bool,
    pub len: u8,
    pub t_spin: TSpin,
    pub garbage_lines: VecDeque<i8>,
    pub target_player_id: Option<PlayerId>,
    pub random_bag: RandomBag,
}

#[derive(Bundle, Default)]
pub struct LocalFieldBundle {
    pub local_field: LocalField,
    pub drop_timer: DropTimer,
    pub lock_down_timer: LockDownTimer,
    pub target_change_timer: TargetChangeTimer,
}

impl Default for LocalField {
    fn default() -> Self {
        Self {
            can_back_to_back: false,
            len: 0,
            t_spin: TSpin::default(),
            garbage_lines: VecDeque::new(),
            target_player_id: None,
            random_bag: RandomBag::new(),
        }
    }
}

pub fn handle_receive_garbage(
    mut receive_garbage_events: EventReader<ReceiveGarbageEvent>,
    mut local_field_query: Query<&mut LocalField>,
) {
    let Ok(mut local_field) = local_field_query.get_single_mut() else { return; };
    for ReceiveGarbageEvent(lines) in receive_garbage_events.iter() {
        local_field.garbage_lines.push_back(*lines);
    }
}

pub fn garbage_line_system(
    mut commands: Commands,
    garbage_line_query: Query<Entity, With<GarbageLine>>,
    mut local_field_query: Query<(Entity, &LocalField)>,
) {
    for garbage_line_entity in garbage_line_query.iter() {
        commands.entity(garbage_line_entity).despawn_recursive();
    }

    let Ok((local_field_entity, local_field)) = local_field_query.get_single_mut() else { return; };
    commands.entity(local_field_entity).with_children(|parent| {
        for (line, offset_y) in local_field.garbage_lines.iter().scan(0, |state, &line| {
            let offset_y = *state;
            *state += line;
            Some((line, offset_y))
        }) {
            let translation = Vec3::new(
                GARBAGE_LINE_START_X,
                GARBAGE_LINE_START_Y
                    + (offset_y as f32 * BLOCK_SIZE)
                    + (line as f32 * BLOCK_SIZE / 2.0),
                0.0,
            );
            let size = Vec2::new(
                GARBAGE_LINE_WIDTH,
                line as f32 * BLOCK_SIZE - GARBAGE_LINE_INSET,
            );

            parent.spawn((
                GarbageLine,
                SpriteBundle {
                    transform: Transform::from_translation(translation),
                    sprite: Sprite {
                        color: GARBAGE_LINE_COLOR,
                        custom_size: Some(size),
                        ..default()
                    },
                    ..default()
                },
            ));
        }
    });
}
