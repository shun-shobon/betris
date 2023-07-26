use bevy::prelude::*;

use crate::{
    field::{
        local::LocalField,
        timer::{DropTimer, LockDownTimer, DROP_INTERVAL, SOFT_DROP_INTERVAL},
        Field,
    },
    mino::{event::PlaceMinoEvent, shape::Shape, t_spin::TSpin, Angle, Mino},
    pos,
    position::Position,
};

#[derive(Debug, Event)]
pub enum MoveEvent {
    Move(Direction),
    Rotate(Direction),
    HardDrop,
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
            Self::Left => pos!(-1, 0),
            Self::Right => pos!(1, 0),
            Self::Down => pos!(0, -1),
        }
    }
}

pub fn handle_move(
    mut move_events: EventReader<MoveEvent>,
    mut mino_query: Query<&mut Mino>,
    mut field_query: Query<
        (&Field, &mut LocalField, &mut DropTimer, &mut LockDownTimer),
        With<Field>,
    >,
    mut place_mino_events: EventWriter<PlaceMinoEvent>,
) {
    for event in move_events.iter() {
        match event {
            MoveEvent::Move(direction) => {
                let Ok(mut mino) = mino_query.get_single_mut() else { continue; };
                let Ok((field, mut local_field, _, mut lock_down_timer)) = field_query.get_single_mut() else { continue; };

                if field.blocks.can_place_mino(
                    mino.pos + direction.move_delta(),
                    mino.shape,
                    mino.angle,
                ) {
                    mino.pos += direction.move_delta();

                    local_field.t_spin = TSpin::None;
                    lock_down_timer.0.reset();
                }
            }
            MoveEvent::Rotate(direction) => {
                let Ok(mut mino) = mino_query.get_single_mut() else { continue; };
                let Ok((field, mut local_field, _, mut lock_down_timer)) = field_query.get_single_mut() else { continue; };

                let new_angle = get_new_angle(mino.angle, *direction);
                let deltas = get_srs_deltas(mino.angle, new_angle, mino.shape);

                let delta = deltas.iter().find(|&&delta| {
                    field
                        .blocks
                        .can_place_mino(mino.pos + delta, mino.shape, new_angle)
                });
                if let Some(&delta) = delta {
                    mino.pos += delta;
                    mino.angle = new_angle;

                    local_field.t_spin.update(&mino, field, delta);
                    lock_down_timer.0.reset();
                }
            }
            MoveEvent::HardDrop => {
                let Ok(mut mino) = mino_query.get_single_mut() else { continue; };
                let Ok((field, _, _, mut lock_down_timer)) = field_query.get_single_mut() else { continue; };

                let mut delta = pos!(0, 0);
                while field.blocks.can_place_mino(
                    mino.pos + delta + pos!(0, -1),
                    mino.shape,
                    mino.angle,
                ) {
                    delta += pos!(0, -1);
                }

                mino.pos += delta;
                lock_down_timer.0.reset();
                place_mino_events.send(PlaceMinoEvent);
            }
            MoveEvent::StartSoftDrop => {
                let Ok((_, _, mut drop_timer, _)) = field_query.get_single_mut() else { continue; };
                drop_timer.0.set_duration(SOFT_DROP_INTERVAL);
            }
            MoveEvent::StopSoftDrop => {
                let Ok((_, _, mut drop_timer, _)) = field_query.get_single_mut() else { continue; };
                drop_timer.0.set_duration(DROP_INTERVAL);
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

fn get_srs_deltas(angle: Angle, new_angle: Angle, shape: Shape) -> &'static [Position] {
    use Angle::*;

    if shape != Shape::I {
        match (angle, new_angle) {
            (Deg0, Deg90) => &SRS_DELTAS_0_TO_90,
            (Deg90, Deg0) => &SRS_DELTAS_90_TO_0,
            (Deg90, Deg180) => &SRS_DELTAS_90_TO_180,
            (Deg180, Deg90) => &SRS_DELTAS_180_TO_90,
            (Deg180, Deg270) => &SRS_DELTAS_180_TO_270,
            (Deg270, Deg180) => &SRS_DELTAS_270_TO_180,
            (Deg270, Deg0) => &SRS_DELTAS_270_TO_0,
            (Deg0, Deg270) => &SRS_DELTAS_0_TO_270,
            (_, _) => unreachable!(),
        }
    } else {
        match (angle, new_angle) {
            (Deg0, Deg90) => &SRS_DELTAS_0_TO_90_I,
            (Deg90, Deg0) => &SRS_DELTAS_90_TO_0_I,
            (Deg90, Deg180) => &SRS_DELTAS_90_TO_180_I,
            (Deg180, Deg90) => &SRS_DELTAS_180_TO_90_I,
            (Deg180, Deg270) => &SRS_DELTAS_180_TO_270_I,
            (Deg270, Deg180) => &SRS_DELTAS_270_TO_180_I,
            (Deg270, Deg0) => &SRS_DELTAS_270_TO_0_I,
            (Deg0, Deg270) => &SRS_DELTAS_0_TO_270_I,
            (_, _) => unreachable!(),
        }
    }
}

type SRSDeltas = [Position; 5];

static SRS_DELTAS_0_TO_90: SRSDeltas = pos![(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)];
static SRS_DELTAS_90_TO_0: SRSDeltas = pos![(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)];
static SRS_DELTAS_90_TO_180: SRSDeltas = pos![(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)];
static SRS_DELTAS_180_TO_90: SRSDeltas = pos![(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)];
static SRS_DELTAS_180_TO_270: SRSDeltas = pos![(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)];
static SRS_DELTAS_270_TO_180: SRSDeltas = pos![(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)];
static SRS_DELTAS_270_TO_0: SRSDeltas = pos![(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)];
static SRS_DELTAS_0_TO_270: SRSDeltas = pos![(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)];

static SRS_DELTAS_0_TO_90_I: SRSDeltas = pos![(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)];
static SRS_DELTAS_90_TO_0_I: SRSDeltas = pos![(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)];
static SRS_DELTAS_90_TO_180_I: SRSDeltas = pos![(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)];

static SRS_DELTAS_180_TO_90_I: SRSDeltas = pos![(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)];
static SRS_DELTAS_180_TO_270_I: SRSDeltas = pos![(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)];
static SRS_DELTAS_270_TO_180_I: SRSDeltas = pos![(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)];
static SRS_DELTAS_270_TO_0_I: SRSDeltas = pos![(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)];
static SRS_DELTAS_0_TO_270_I: SRSDeltas = pos![(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)];
