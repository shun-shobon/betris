use bevy::prelude::*;

use crate::{
    block::Block,
    field::{FIELD_HEIGHT, FIELD_WIDTH},
    mino::MinoPosition,
    position::Position,
};

#[derive(Debug, Event)]
pub struct MoveEvent(pub Entity, pub Direction);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Left,
    Right,
    Down,
}

pub fn handle_move_event(
    mut move_events: EventReader<MoveEvent>,
    mut mino_query: Query<(&mut MinoPosition, &Children, &Parent)>,
    blocks_query: Query<(&Block, &Parent)>,
) {
    for event in move_events.iter() {
        let MoveEvent(field_entity, direction) = event;
        let Some((mut mino_pos, mino_block_entities, _)) = mino_query.iter_mut().find(|(_, _, parent)| parent.get() == *field_entity) else { continue; };
        let field_blocks = blocks_query
            .iter()
            .filter(|(_, parent)| parent.get() == *field_entity)
            .map(|(block, _)| block.position)
            .collect::<Vec<_>>();

        let delta = match direction {
            Direction::Left => Position::new(-1, 0),
            Direction::Right => Position::new(1, 0),
            Direction::Down => Position::new(0, 1),
        };

        let mut collision = false;
        for &mino_block_entity in mino_block_entities.iter() {
            let mino_block = blocks_query
                .get(mino_block_entity)
                .map(|(block, _)| block)
                .unwrap();
            let mino_block_new_pos = mino_pos.0 + mino_block.position + delta;
            if mino_block_new_pos.x < 0
                || FIELD_WIDTH <= mino_block_new_pos.x
                || mino_block_new_pos.y < 0
                || FIELD_HEIGHT <= mino_block_new_pos.y
                || field_blocks.contains(&mino_block_new_pos)
            {
                collision = true;
                break;
            }
        }

        if !collision {
            mino_pos.0 += delta;
        }
    }
}
