use super::{
    block::{BLOCK_INSET, BLOCK_SIZE},
    next::{NextQueue, QUEUE_SIZE},
    timer::{DropTimer, LockDownTimer, TargetChangeTimer},
    FIELD_BACKGROUND_COLOR, FIELD_PIXEL_HEIGHT, FIELD_PIXEL_WIDTH,
};
use crate::{
    mino::{event::SpawnMinoEvent, shape::Shape, t_spin::TSpin, Angle, Mino},
    net::PlayerId,
    position::Position,
};
use bevy::{prelude::*, sprite::Anchor};

static GARBAGE_WARN_BAR_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);
static GARBAGE_WARN_BAR_WIDTH: f32 = 20.0;
static GARBAGE_WARN_BAR_INSET: f32 = 4.0;
static GARBAGE_WARN_BAR_START_X: f32 =
    -FIELD_PIXEL_WIDTH / 2.0 - GARBAGE_WARN_BAR_WIDTH / 2.0 - GARBAGE_WARN_BAR_INSET;
static GARBAGE_WARN_BAR_START_Y: f32 = -FIELD_PIXEL_HEIGHT / 2.0;

pub const NEXT_HOLD_BLOCK_SIZE: f32 = BLOCK_SIZE * 0.6;
pub const NEXT_HOLD_BLOCK_INSET: f32 = BLOCK_INSET * 0.6;
pub const NEXT_HOLD_BG_PADDING: f32 = NEXT_HOLD_BLOCK_SIZE * 0.5;
pub const NEXT_HOLD_BG_WIDTH: f32 = NEXT_HOLD_BLOCK_SIZE * 4.0 + NEXT_HOLD_BG_PADDING * 2.0;
pub const NEXT_HOLD_BG_HEIGHT: f32 = NEXT_HOLD_BLOCK_SIZE * 2.0 + NEXT_HOLD_BG_PADDING * 2.0;
pub const NEXT_START_X: f32 =
    FIELD_PIXEL_WIDTH / 2.0 + NEXT_HOLD_BG_PADDING + NEXT_HOLD_BG_WIDTH / 2.0;
pub const HOLD_START_X: f32 = -NEXT_START_X;
pub const NEXT_HOLD_BG_START_Y: f32 = FIELD_PIXEL_HEIGHT / 2.0 - NEXT_HOLD_BG_HEIGHT / 2.0;

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

#[derive(Component)]
pub struct NextHoldBlock;

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
    let Ok(mut local_field) = local_field_query.get_single_mut() else {
        return;
    };
    for ReceiveGarbageEvent(lines) in receive_garbage_events.read() {
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
    let Ok(mut local_field) = local_field_query.get_single_mut() else {
        return;
    };
    for _ in events.read() {
        if local_field.is_hold_used {
            continue;
        }
        local_field.is_hold_used = true;

        let Ok((mino_entity, mino)) = mino_query.get_single_mut() else {
            continue;
        };
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
    let Ok((mut sprite, mut visibility)) = garbage_line_query.get_single_mut() else {
        return;
    };
    let Ok(local_field) = local_field_query.get_single() else {
        return;
    };

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

pub fn next_hold_block_system(
    mut commands: Commands,
    block_query: Query<Entity, With<NextHoldBlock>>,
    field_query: Query<(Entity, &LocalField), With<LocalField>>,
) {
    for entity in block_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let Ok((field_entity, field)) = field_query.get_single() else {
        return;
    };
    commands.entity(field_entity).with_children(|parent| {
        for (i, shape) in field.next_queue.queue().iter().enumerate() {
            let base = next_pos(i);

            for &pos in shape.blocks(Angle::default()) {
                let translation = base + pos_to_translation(pos, shape.offset_y(), shape.width());

                let bundle = create_next_hold_block_bundle(translation, shape.color());
                parent.spawn(bundle);
            }
        }

        if let Some(shape) = field.hold {
            let base = Vec3::new(HOLD_START_X, NEXT_HOLD_BG_START_Y, 0.0);

            for &pos in shape.blocks(Angle::default()) {
                let translation = base + pos_to_translation(pos, shape.offset_y(), shape.width());

                let bundle = create_next_hold_block_bundle(translation, shape.color());
                parent.spawn(bundle);
            }
        }
    });
}

pub fn spawn_next_hold_background(parent: &mut ChildBuilder) {
    let next_hold_sprite = Sprite {
        color: FIELD_BACKGROUND_COLOR,
        custom_size: Some(Vec2::new(NEXT_HOLD_BG_WIDTH, NEXT_HOLD_BG_HEIGHT)),
        ..default()
    };

    for i in 0..QUEUE_SIZE {
        let translation = next_pos(i);

        parent.spawn(SpriteBundle {
            transform: Transform::from_translation(translation),
            sprite: next_hold_sprite.clone(),
            ..default()
        });
    }

    parent.spawn(SpriteBundle {
        transform: Transform::from_translation(Vec3::new(HOLD_START_X, NEXT_HOLD_BG_START_Y, 0.0)),
        sprite: next_hold_sprite,
        ..default()
    });
}

fn create_next_hold_block_bundle(translation: Vec3, color: Color) -> impl Bundle {
    (
        NextHoldBlock,
        SpriteBundle {
            transform: Transform::from_translation(translation),
            sprite: Sprite {
                color,
                anchor: Anchor::BottomLeft,
                custom_size: Some(Vec2::new(
                    NEXT_HOLD_BLOCK_SIZE - NEXT_HOLD_BLOCK_INSET,
                    NEXT_HOLD_BLOCK_SIZE - NEXT_HOLD_BLOCK_INSET,
                )),
                ..default()
            },
            ..default()
        },
    )
}

#[allow(clippy::cast_precision_loss)]
fn next_pos(i: usize) -> Vec3 {
    Vec3::new(
        NEXT_START_X,
        NEXT_HOLD_BG_START_Y - (NEXT_HOLD_BG_HEIGHT + NEXT_HOLD_BG_PADDING) * i as f32,
        0.0,
    )
}

fn pos_to_translation(pos: Position, offset_y: i8, width: i8) -> Vec3 {
    let x = (pos.x as f32 - width as f32 / 2.0) * NEXT_HOLD_BLOCK_SIZE;
    let y = (pos.y - offset_y - 1) as f32 * NEXT_HOLD_BLOCK_SIZE;

    Vec3::new(x, y, 0.0)
}
