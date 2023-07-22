use bevy::prelude::*;

use crate::{
    block::Block,
    field::{Field, LocalField, FIELD_WIDTH},
    mino::{Angle, Mino, MinoPosition},
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
    mut mino_query: Query<(&mut Mino, &mut MinoPosition, &Children)>,
    mut field_query: Query<(Entity, &mut LocalField), With<Field>>,
    blocks_query: Query<(&Block, &Parent)>,
) {
    for event in move_events.iter() {
        match event {
            MoveEvent::Move(direction) => {
                let Ok((_, mut mino_pos, mino_block_entities )) = mino_query.get_single_mut() else { continue; };
                let Ok((field_entity, mut local_field)) = field_query.get_single_mut() else { continue; };
                let field_blocks = blocks_query
                    .iter()
                    .filter(|(_, parent)| parent.get() == field_entity)
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
                    local_field.lock_down_timer.reset();
                    local_field.lock_down_timer.pause();
                }

                let is_landed = is_collision(
                    mino_block_entities,
                    &blocks_query,
                    &mino_pos,
                    Direction::Down,
                    &field_blocks,
                );
                if is_landed {
                    local_field.drop_timer.reset();
                    local_field.lock_down_timer.unpause();
                }
            }
            MoveEvent::Rotate(_direction) => {
                todo!()
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

#[allow(dead_code)]
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
