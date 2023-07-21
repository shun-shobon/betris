use super::{Mino, MinoPosition};
use crate::{
    block::Block,
    field::{Field, LocalField},
};
use bevy::prelude::*;

#[derive(Event)]
pub struct SpawnMinoEvent;

#[derive(Event)]
pub struct PlaceMinoEvent(pub Entity);

pub fn handle_spawn_mino(
    mut commands: Commands,
    mut spawn_mino_events: EventReader<SpawnMinoEvent>,
    mut field_query: Query<(Entity, &Field, &mut LocalField)>,
) {
    for SpawnMinoEvent in spawn_mino_events.iter() {
        let Ok((field_entity, field, mut local_field)) = field_query.get_single_mut() else { continue; };

        let mino_type = local_field.random_bag.next().unwrap();

        let mino_entity = Mino::spawn(&mut commands, mino_type, field.block_size);
        commands.entity(field_entity).add_child(mino_entity);
    }
}

pub fn handle_place_mino(
    mut commands: Commands,
    mut place_mino_events: EventReader<PlaceMinoEvent>,
    mut spawn_mino_event_writer: EventWriter<SpawnMinoEvent>,
    mino_query: Query<(Entity, &MinoPosition, &Children, &Parent), With<Mino>>,
    field_query: Query<&Field>,
    block_query: Query<(&Block, &Sprite, &Parent)>,
) {
    for PlaceMinoEvent(field_entity) in place_mino_events.iter() {
        let Some((mino_entity, mino_pos, mino_block_entities, _)) = mino_query.iter().find(|(_, _, _, parent)| parent.get() == *field_entity) else { continue; };
        let Ok(field) = field_query.get(*field_entity) else { continue; };

        commands.entity(*field_entity).with_children(|parent| {
            for &block_entity in mino_block_entities.iter() {
                let (block, block_sprite, _) = block_query.get(block_entity).unwrap();
                let block_pos = block.position + mino_pos.0;
                Block::spawn_with_parent(parent, block_sprite.color, field.block_size, block_pos);
            }
        });

        commands.entity(mino_entity).despawn_recursive();
        spawn_mino_event_writer.send(SpawnMinoEvent);
    }
}
