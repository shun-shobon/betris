use super::{shape::MinoShape, Angle, Mino};
use crate::{
    block::Block,
    field::{Field, LocalField},
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
    for SpawnMinoEvent in spawn_mino_events.iter() {
        let Ok((field_entity, field, mut local_field)) = field_query.get_single_mut() else { continue; };

        let mino_shape = local_field.random_bag.next().unwrap();

        let mino_entity = Mino::new(mino_shape).spawn(&mut commands, field.block_size);
        commands.entity(field_entity).add_child(mino_entity);
    }
}

pub fn handle_place_mino(
    mut commands: Commands,
    mut place_mino_events: EventReader<PlaceMinoEvent>,
    field_query: Query<(Entity, &Field)>,
) {
    for event in place_mino_events.iter() {
        let Some((field_entity, field)) = field_query.iter().find(|(_, field)| field.player_id == event.player_id) else { continue; };

        commands.entity(field_entity).with_children(|parent| {
            for &block_pos in event.shape.blocks(event.angle).iter() {
                let block_pos = block_pos + event.pos;
                Block::spawn_with_parent(parent, event.shape.color(), field.block_size, block_pos);
            }
        });
    }
}
