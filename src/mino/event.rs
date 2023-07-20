use bevy::prelude::*;

use crate::field::Field;

use super::{shape::MinoShape, Mino};

#[derive(Event)]
pub struct SpwanMinoEvent(pub u32);

pub fn handle_spwan_mino(
    mut commands: Commands,
    mut spwan_mino_events: EventReader<SpwanMinoEvent>,
    field_query: Query<(Entity, &Field)>,
) {
    for SpwanMinoEvent(id) in spwan_mino_events.iter() {
        let Some((field_entity, field)) = field_query.iter().find(|(_, field)| field.id == *id) else { continue; };

        let mino_type = MinoShape::T;

        let mino_entity = Mino::spawn(&mut commands, mino_type, field.block_size);
        commands.entity(field_entity).add_child(mino_entity);
    }
}
