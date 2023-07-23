use super::{shape::MinoShape, Angle, Mino};
use crate::{
    field::{Field, FieldBlock, LocalField, FIELD_MAX_HEIGHT, FIELD_WIDTH},
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
    mut place_mino_events: EventReader<PlaceMinoEvent>,
    mut field_query: Query<&mut Field>,
) {
    for event in place_mino_events.iter() {
        let Some(mut field) = field_query.iter_mut().find(|field| field.player_id == event.player_id) else { continue; };

        for &block_pos in event.shape.blocks(event.angle).iter() {
            let pos = block_pos + event.pos;
            field.lines[pos.y as usize][pos.x as usize] = event.shape.into();
        }

        let clear_lines = field
            .lines
            .iter()
            .enumerate()
            .filter(|(_, line)| line.iter().all(|field_block| !field_block.is_empty()))
            .map(|(y, _)| y as i8)
            .rev()
            .collect::<Vec<_>>();

        for &clear_y in clear_lines.iter() {
            for y in clear_y..(FIELD_MAX_HEIGHT - 1) {
                field.lines[y as usize] = field.lines[(y + 1) as usize];
            }
            field.lines[(FIELD_MAX_HEIGHT - 1) as usize] =
                [FieldBlock::default(); FIELD_WIDTH as usize];
        }
    }
}
