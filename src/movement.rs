use bevy::prelude::*;

use crate::{
    block::Block,
    field::{Field, FIELD_WIDTH},
    mino::{Angle, Mino, MinoPosition},
    position::Position,
    timer::{DropTimer, LockDownTimer, DROP_INTERVAL, SOFT_DROP_INTERVAL},
};

#[derive(Debug, Event)]
pub enum MoveEvent {
    Move(Entity, Direction),
    Rotate(Entity, Direction),
    StartSoftDrop(Entity),
    StopSoftDrop(Entity),
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
    mut mino_query: Query<(&mut Mino, &mut MinoPosition, &Children, &Parent)>,
    mut field_query: Query<(&mut DropTimer, &mut LockDownTimer), With<Field>>,
    blocks_query: Query<(&Block, &Parent)>,
) {
    for event in move_events.iter() {
        match event {
            MoveEvent::Move(field_entity, direction) => {
                let Some((_, mut mino_pos, mino_block_entities, _)) = mino_query.iter_mut().find(|(_, _, _, parent)| parent.get() == *field_entity) else { continue; };
                let Ok((mut drop_timer, mut lock_down_timer)) = field_query.get_mut(*field_entity) else { continue; };
                let field_blocks = blocks_query
                    .iter()
                    .filter(|(_, parent)| parent.get() == *field_entity)
                    .map(|(block, _)| block.position)
                    .collect::<Vec<_>>();

                let collision = is_collision(
                    mino_block_entities,
                    &blocks_query,
                    &mino_pos,
                    *direction,
                    &field_blocks,
                );

                if !collision {
                    mino_pos.0 += direction.move_delta();
                    lock_down_timer.0.reset();
                    lock_down_timer.0.pause();
                }

                let is_landed = is_collision(
                    mino_block_entities,
                    &blocks_query,
                    &mino_pos,
                    Direction::Down,
                    &field_blocks,
                );
                if is_landed {
                    drop_timer.0.reset();
                    lock_down_timer.0.unpause();
                }
            }
            MoveEvent::Rotate(field_entity, direction) => {
                todo!()
            }
            MoveEvent::StartSoftDrop(field_entity) => {
                let Ok((mut drop_timer, _)) = field_query.get_mut(*field_entity) else { continue; };
                drop_timer.0.set_duration(SOFT_DROP_INTERVAL);
            }
            MoveEvent::StopSoftDrop(field_entity) => {
                let Ok((mut drop_timer, _)) = field_query.get_mut(*field_entity) else { continue; };
                drop_timer.0.set_duration(DROP_INTERVAL);
            }
        }
    }
}

fn get_new_angle(angle: Angle, direction: Direction) -> Angle {
    use Angle::*;

    match (angle, direction) {
        (Deg0, Direction::Left) => Deg270,
        (Deg0, Direction::Right) => Deg90,
        (Deg90, Direction::Left) => Deg0,
        (Deg90, Direction::Right) => Deg180,
        (Deg180, Direction::Left) => Deg90,
        (Deg180, Direction::Right) => Deg270,
        (Deg270, Direction::Left) => Deg180,
        (Deg270, Direction::Right) => Deg0,
        (_, Direction::Down) => unreachable!(),
    }
}

fn is_collision(
    mino_block_entities: &Children,
    blocks_query: &Query<(&Block, &Parent)>,
    mino_pos: &MinoPosition,
    direction: Direction,
    field_blocks: &[Position],
) -> bool {
    let mut collision = false;

    for &mino_block_entity in mino_block_entities.iter() {
        let mino_block = blocks_query
            .get(mino_block_entity)
            .map(|(block, _)| block)
            .unwrap();
        let mino_block_new_pos = mino_pos.0 + mino_block.position + direction.move_delta();
        if mino_block_new_pos.x < 0
            || FIELD_WIDTH <= mino_block_new_pos.x
            || mino_block_new_pos.y < 0
            || field_blocks.contains(&mino_block_new_pos)
        {
            collision = true;
            break;
        }
    }

    collision
}
