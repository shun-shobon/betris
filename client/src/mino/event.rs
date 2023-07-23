use std::cmp::Ordering;

use super::{shape::MinoShape, Angle, Mino};
use crate::{
    block::Block,
    field::{Field, LocalField, FIELD_MAX_HEIGHT, FIELD_WIDTH},
    net::PlayerId,
    position::Position,
};
use bevy::prelude::*;

#[derive(Event)]
pub struct SpawnMinoEvent;

#[derive(Debug, Clone, Copy, Event)]
pub struct PlaceMinoEvent {
    pub player_id: PlayerId,
    pub pos: Position,
    pub angle: Angle,
    pub shape: MinoShape,
}

#[derive(Debug, Clone, Event)]
pub struct ClearLineEvent {
    pub player_id: PlayerId,
    pub clear_lines: Vec<i8>,
}

pub fn handle_spawn_mino(
    mut commands: Commands,
    mut spawn_mino_events: EventReader<SpawnMinoEvent>,
    mut field_query: Query<(Entity, &Field, &mut LocalField)>,
) {
    for _ in spawn_mino_events.iter() {
        let Ok((field_entity, field, mut local_field)) = field_query.get_single_mut() else { continue; };

        let mino_shape = local_field.random_bag.next().unwrap();

        let mino_entity = Mino::new(mino_shape).spawn(&mut commands, field.block_size);
        commands.entity(field_entity).add_child(mino_entity);
    }
}

pub fn handle_place_mino(
    mut commands: Commands,
    mut place_mino_events: EventReader<PlaceMinoEvent>,
    mut field_query: Query<(Entity, &mut Field)>,
    mut clear_line_events: EventWriter<ClearLineEvent>,
) {
    for event in place_mino_events.iter() {
        let Some((field_entity, mut field)) = field_query.iter_mut().find(|(_, field)| field.player_id == event.player_id) else { continue; };

        commands.entity(field_entity).with_children(|parent| {
            for &block_pos in event.shape.blocks(event.angle).iter() {
                let pos = block_pos + event.pos;
                field.lines[pos.y as usize][pos.x as usize] = true;
                Block::spawn_with_parent(parent, event.shape.color(), field.block_size, pos);
            }
        });

        let clear_lines = field
            .lines
            .iter()
            .enumerate()
            .filter(|(_, line)| line.iter().all(|&is_block| is_block))
            .map(|(y, _)| y as i8)
            .rev()
            .collect::<Vec<_>>();

        if !clear_lines.is_empty() {
            clear_line_events.send(ClearLineEvent {
                player_id: field.player_id,
                clear_lines,
            });
        }
    }
}

pub fn handle_clear_line(
    mut commands: Commands,
    mut clear_line_events: EventReader<ClearLineEvent>,
    mut field_query: Query<(Entity, &mut Field)>,
    mut block_query: Query<(Entity, &mut Block, &Parent)>,
) {
    for event in clear_line_events.iter() {
        let Some((field_entity, mut field)) = field_query.iter_mut().find(|(_, field)| field.player_id == event.player_id) else { continue; };

        let mut field_blocks = block_query
            .iter_mut()
            .filter(|(_, _, parent)| parent.get() == field_entity)
            .map(|(block_entity, block, _)| (block_entity, block))
            .collect::<Vec<_>>();

        for &clear_y in event.clear_lines.iter() {
            for y in clear_y..(FIELD_MAX_HEIGHT - 1) {
                field.lines[y as usize] = field.lines[(y + 1) as usize];
            }
            field.lines[(FIELD_MAX_HEIGHT - 1) as usize] = [false; FIELD_WIDTH as usize];

            for (block_entity, block) in field_blocks.iter_mut() {
                match clear_y.cmp(&block.position.y) {
                    Ordering::Equal => commands.entity(*block_entity).despawn_recursive(),
                    Ordering::Less => block.position.y -= 1,
                    _ => {}
                }
            }
        }
    }
}
