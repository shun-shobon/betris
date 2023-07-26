use super::{
    block::BLOCK_SIZE,
    next::NextQueue,
    timer::{DropTimer, LockDownTimer, TargetChangeTimer},
    FIELD_PIXEL_WIDTH,
};
use crate::{
    mino::{event::SpawnMinoEvent, shape::Shape, t_spin::TSpin, Mino},
    net::PlayerId,
};
use bevy::{prelude::*, sprite::Anchor};

static GARBAGE_WARN_BAR_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);
static GARBAGE_WARN_BAR_WIDTH: f32 = 20.0;
static GARBAGE_WARN_BAR_INSET: f32 = 4.0;
static GARBAGE_WARN_BAR_START_X: f32 =
    -FIELD_PIXEL_WIDTH / 2.0 - GARBAGE_WARN_BAR_WIDTH / 2.0 - GARBAGE_WARN_BAR_INSET;
static GARBAGE_WARN_BAR_START_Y: f32 = -FIELD_PIXEL_WIDTH / 2.0;

#[derive(Debug, Event)]
pub struct ReceiveGarbageEvent(pub u8);

#[derive(Debug, Event)]
pub struct HoldEvent;

#[derive(Component, Default)]
pub struct LocalField {
    pub can_back_to_back: bool,
    pub combo: u8,
    pub t_spin: TSpin,
    pub garbage_amount: u8,
    pub target_player_id: Option<PlayerId>,
    pub next_queue: NextQueue,
    pub hold: Option<Shape>,
    pub is_hold_used: bool,
}

#[derive(Bundle, Default)]
pub struct LocalFieldBundle {
    pub local_field: LocalField,
    pub drop_timer: DropTimer,
    pub lock_down_timer: LockDownTimer,
    pub target_change_timer: TargetChangeTimer,
}

#[derive(Component)]
pub struct GarbageWarningBar;

impl GarbageWarningBar {
    pub fn spawn(parent: &mut ChildBuilder) {
        parent.spawn((
            SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    GARBAGE_WARN_BAR_START_X,
                    GARBAGE_WARN_BAR_START_Y,
                    0.0,
                )),
                sprite: Sprite {
                    anchor: Anchor::BottomCenter,
                    color: GARBAGE_WARN_BAR_COLOR,
                    ..default()
                },
                ..default()
            },
            GarbageWarningBar,
        ));
    }
}

pub fn handle_receive_garbage(
    mut receive_garbage_events: EventReader<ReceiveGarbageEvent>,
    mut local_field_query: Query<&mut LocalField>,
) {
    let Ok(mut local_field) = local_field_query.get_single_mut() else { return; };
    for ReceiveGarbageEvent(lines) in receive_garbage_events.iter() {
        local_field.garbage_amount += lines;
    }
}

pub fn handle_hold(
    mut commands: Commands,
    mut events: EventReader<HoldEvent>,
    mut local_field_query: Query<&mut LocalField>,
    mut mino_query: Query<(Entity, &Mino)>,
    mut spawn_mino_events: EventWriter<SpawnMinoEvent>,
) {
    let Ok(mut local_field) = local_field_query.get_single_mut() else { return; };
    for _ in events.iter() {
        if local_field.is_hold_used {
            continue;
        }
        local_field.is_hold_used = true;

        let Ok((mino_entity, mino)) = mino_query.get_single_mut() else { continue; };
        commands.entity(mino_entity).despawn_recursive();
        let next_shape = if let Some(shape) = local_field.hold {
            shape
        } else {
            local_field.next_queue.pop()
        };
        local_field.hold = Some(mino.shape);

        spawn_mino_events.send(SpawnMinoEvent(next_shape));
    }
}

pub fn garbage_warning_bar_system(
    mut garbage_line_query: Query<(&mut Sprite, &mut Visibility), With<GarbageWarningBar>>,
    local_field_query: Query<&LocalField>,
) {
    let Ok((mut sprite, mut visibility)) = garbage_line_query.get_single_mut() else { return; };
    let Ok(local_field) = local_field_query.get_single() else { return; };

    *visibility = if local_field.garbage_amount == 0 {
        Visibility::Hidden
    } else {
        Visibility::Inherited
    };

    sprite.custom_size = Some(Vec2::new(
        GARBAGE_WARN_BAR_WIDTH,
        local_field.garbage_amount as f32 * BLOCK_SIZE - GARBAGE_WARN_BAR_INSET,
    ));
}
