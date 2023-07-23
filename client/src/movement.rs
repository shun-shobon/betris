use bevy::prelude::*;

use crate::{
    field::{Field, LocalField, FIELD_MAX_HEIGHT, FIELD_WIDTH},
    mino::{shape::MinoShape, Angle, Mino, MinoPosition},
    position::Position,
    timer::{DROP_INTERVAL, SOFT_DROP_INTERVAL},
};

#[derive(Debug, Event)]
pub enum MoveEvent {
    Move(Direction),
    Rotate(Direction),
    StartSoftDrop,
    StopSoftDrop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Left,
    Right,
    Down,
}

impl Direction {
    pub fn move_delta(&self) -> Position {
        match self {
            Self::Left => Position::new(-1, 0),
            Self::Right => Position::new(1, 0),
            Self::Down => Position::new(0, -1),
        }
    }
}

pub fn handle_move_event(
    mut move_events: EventReader<MoveEvent>,
    mut mino_query: Query<(&mut Mino, &mut MinoPosition)>,
    mut field_query: Query<(&Field, &mut LocalField), With<Field>>,
) {
    for event in move_events.iter() {
        match event {
            MoveEvent::Move(direction) => {
                let Ok((mino, mut mino_pos)) = mino_query.get_single_mut() else { continue; };
                let Ok((field, mut local_field)) = field_query.get_single_mut() else { continue; };

                let collision = is_collision(
                    mino.shape.blocks(mino.angle),
                    &mino_pos,
                    direction.move_delta(),
                    field,
                );

                if !collision {
                    mino_pos.0 += direction.move_delta();
                    local_field.lock_down_timer.reset();
                    local_field.lock_down_timer.pause();
                }

                let is_landed = is_collision(
                    mino.shape.blocks(mino.angle),
                    &mino_pos,
                    Direction::Down.move_delta(),
                    field,
                );
                if is_landed {
                    local_field.drop_timer.reset();
                    local_field.lock_down_timer.unpause();
                }
            }
            MoveEvent::Rotate(direction) => {
                let Ok((mut mino, mut mino_pos)) = mino_query.get_single_mut() else { continue; };
                let Ok((field, mut local_field)) = field_query.get_single_mut() else { continue; };

                let new_angle = get_new_angle(mino.angle, *direction);
                let deltas = get_srs_deltas(mino.angle, new_angle, mino.shape);

                if let Some(delta) = deltas.iter().find_map(|delta| {
                    if !is_collision(mino.shape.blocks(new_angle), &mino_pos, *delta, field) {
                        Some(*delta)
                    } else {
                        None
                    }
                }) {
                    mino_pos.0 += delta;
                    mino.angle = new_angle;
                    local_field.lock_down_timer.reset();
                    local_field.lock_down_timer.pause();
                }
            }
            MoveEvent::StartSoftDrop => {
                let Ok((_, mut local_field)) = field_query.get_single_mut() else { continue; };
                local_field.drop_timer.set_duration(SOFT_DROP_INTERVAL);
            }
            MoveEvent::StopSoftDrop => {
                let Ok((_, mut local_field)) = field_query.get_single_mut() else { continue; };
                local_field.drop_timer.set_duration(DROP_INTERVAL);
            }
        }
    }
}

fn get_new_angle(angle: Angle, direction: Direction) -> Angle {
    use self::Direction::*;
    use Angle::*;

    match (angle, direction) {
        (Deg0, Left) => Deg270,
        (Deg0, Right) => Deg90,
        (Deg90, Left) => Deg0,
        (Deg90, Right) => Deg180,
        (Deg180, Left) => Deg90,
        (Deg180, Right) => Deg270,
        (Deg270, Left) => Deg180,
        (Deg270, Right) => Deg0,
        (_, Down) => unreachable!(),
    }
}

fn get_srs_deltas(angle: Angle, new_angle: Angle, shape: MinoShape) -> &'static [Position] {
    use Angle::*;

    if shape != MinoShape::I {
        match (angle, new_angle) {
            (Deg0, Deg90) => SRS_DELTAS_0_TO_90,
            (Deg90, Deg0) => SRS_DELTAS_90_TO_0,
            (Deg90, Deg180) => SRS_DELTAS_90_TO_180,
            (Deg180, Deg90) => SRS_DELTAS_180_TO_90,
            (Deg180, Deg270) => SRS_DELTAS_180_TO_270,
            (Deg270, Deg180) => SRS_DELTAS_270_TO_180,
            (Deg270, Deg0) => SRS_DELTAS_270_TO_0,
            (Deg0, Deg270) => SRS_DELTAS_0_TO_270,
            (_, _) => unreachable!(),
        }
    } else {
        match (angle, new_angle) {
            (Deg0, Deg90) => SRS_DELTAS_0_TO_90_I_MINO,
            (Deg90, Deg0) => SRS_DELTAS_90_TO_0_I_MINO,
            (Deg90, Deg180) => SRS_DELTAS_90_TO_180_I_MINO,
            (Deg180, Deg90) => SRS_DELTAS_180_TO_90_I_MINO,
            (Deg180, Deg270) => SRS_DELTAS_180_TO_270_I_MINO,
            (Deg270, Deg180) => SRS_DELTAS_270_TO_180_I_MINO,
            (Deg270, Deg0) => SRS_DELTAS_270_TO_0_I_MINO,
            (Deg0, Deg270) => SRS_DELTAS_0_TO_270_I_MINO,
            (_, _) => unreachable!(),
        }
    }
}

fn is_collision(
    mino_blocks: &[Position],
    MinoPosition(mino_pos): &MinoPosition,
    delta: Position,
    field: &Field,
) -> bool {
    !mino_blocks
        .iter()
        .map(|&mino_block_pos| mino_block_pos + *mino_pos + delta)
        .all(|pos| {
            0 <= pos.x
                && pos.x < FIELD_WIDTH
                && 0 <= pos.y
                && pos.y < FIELD_MAX_HEIGHT
                && field.lines[pos.y as usize][pos.x as usize].is_empty()
        })
}

static SRS_DELTAS_0_TO_90: &[Position] = &[
    Position::new(0, 0),
    Position::new(-1, 0),
    Position::new(-1, 1),
    Position::new(0, -2),
    Position::new(-1, -2),
];
static SRS_DELTAS_90_TO_0: &[Position] = &[
    Position::new(0, 0),
    Position::new(1, 0),
    Position::new(1, -1),
    Position::new(0, 2),
    Position::new(1, 2),
];
static SRS_DELTAS_90_TO_180: &[Position] = &[
    Position::new(0, 0),
    Position::new(1, 0),
    Position::new(1, -1),
    Position::new(0, 2),
    Position::new(1, 2),
];
static SRS_DELTAS_180_TO_90: &[Position] = &[
    Position::new(0, 0),
    Position::new(-1, 0),
    Position::new(-1, 1),
    Position::new(0, -2),
    Position::new(-1, -2),
];
static SRS_DELTAS_180_TO_270: &[Position] = &[
    Position::new(0, 0),
    Position::new(1, 0),
    Position::new(1, 1),
    Position::new(0, -2),
    Position::new(1, -2),
];
static SRS_DELTAS_270_TO_180: &[Position] = &[
    Position::new(0, 0),
    Position::new(-1, 0),
    Position::new(-1, -1),
    Position::new(0, 2),
    Position::new(-1, 2),
];
static SRS_DELTAS_270_TO_0: &[Position] = &[
    Position::new(0, 0),
    Position::new(-1, 0),
    Position::new(-1, -1),
    Position::new(0, 2),
    Position::new(-1, 2),
];
static SRS_DELTAS_0_TO_270: &[Position] = &[
    Position::new(0, 0),
    Position::new(1, 0),
    Position::new(1, 1),
    Position::new(0, -2),
    Position::new(1, -2),
];

static SRS_DELTAS_0_TO_90_I_MINO: &[Position] = &[
    Position::new(0, 0),
    Position::new(-2, 0),
    Position::new(1, 0),
    Position::new(-2, -1),
    Position::new(1, 2),
];
static SRS_DELTAS_90_TO_0_I_MINO: &[Position] = &[
    Position::new(0, 0),
    Position::new(2, 0),
    Position::new(-1, 0),
    Position::new(2, 1),
    Position::new(-1, -2),
];
static SRS_DELTAS_90_TO_180_I_MINO: &[Position] = &[
    Position::new(0, 0),
    Position::new(-1, 0),
    Position::new(2, 0),
    Position::new(-1, 2),
    Position::new(2, -1),
];
static SRS_DELTAS_180_TO_90_I_MINO: &[Position] = &[
    Position::new(0, 0),
    Position::new(1, 0),
    Position::new(-2, 0),
    Position::new(1, -2),
    Position::new(-2, 1),
];
static SRS_DELTAS_180_TO_270_I_MINO: &[Position] = &[
    Position::new(0, 0),
    Position::new(2, 0),
    Position::new(-1, 0),
    Position::new(2, 1),
    Position::new(-1, -2),
];
static SRS_DELTAS_270_TO_180_I_MINO: &[Position] = &[
    Position::new(0, 0),
    Position::new(-2, 0),
    Position::new(1, 0),
    Position::new(-2, -1),
    Position::new(1, 2),
];
static SRS_DELTAS_270_TO_0_I_MINO: &[Position] = &[
    Position::new(0, 0),
    Position::new(1, 0),
    Position::new(-2, 0),
    Position::new(1, -2),
    Position::new(-2, 1),
];
static SRS_DELTAS_0_TO_270_I_MINO: &[Position] = &[
    Position::new(0, 0),
    Position::new(-1, 0),
    Position::new(2, 0),
    Position::new(-1, 2),
    Position::new(2, -1),
];
