use super::{Mino, TSpin};
use crate::{
    field::{block::FieldBlock, Field, LocalField, FIELD_MAX_HEIGHT, FIELD_WIDTH},
    net::{LocalSendLinesEvent, PlayerId},
};
use bevy::prelude::*;
use if_chain::if_chain;

#[derive(Event)]
pub struct SpawnMinoEvent;

#[derive(Debug, Clone, Copy, Event)]
pub struct PlaceMinoEvent {
    pub player_id: PlayerId,
    pub mino: Mino,
}

pub fn handle_spawn_mino(
    mut commands: Commands,
    mut spawn_mino_events: EventReader<SpawnMinoEvent>,
    mut field_query: Query<(Entity, &mut LocalField)>,
) {
    for _ in spawn_mino_events.iter() {
        let Ok((field_entity, mut local_field)) = field_query.get_single_mut() else { continue; };

        let mino_shape = local_field.random_bag.next().unwrap();

        let mino_entity = Mino::new(mino_shape).spawn(&mut commands);
        commands.entity(field_entity).add_child(mino_entity);
    }
}

pub fn handle_place_mino(
    mut place_mino_events: EventReader<PlaceMinoEvent>,
    mut field_query: Query<(&mut Field, Option<&mut LocalField>)>,
    mut local_send_line_events: EventWriter<LocalSendLinesEvent>,
) {
    for PlaceMinoEvent { player_id, mino } in place_mino_events.iter() {
        let Some((mut field, local_field)) = field_query.iter_mut().find(|(field, _)| field.player_id == *player_id) else { continue; };

        for &block_pos in mino.shape.blocks(mino.angle).iter() {
            let pos = block_pos + mino.pos;
            field.lines[pos.y as usize][pos.x as usize] = mino.shape.into();
        }

        let clear_lines = field
            .lines
            .iter()
            .enumerate()
            .filter(|(_, line)| line.iter().all(|field_block| !field_block.is_empty()))
            .map(|(y, _)| y as i8)
            .rev()
            .collect::<Vec<_>>();

        clear_line(&mut field, &clear_lines);

        if let Some(mut local_field) = local_field {
            handle_local_field(
                &field,
                &mut local_field,
                clear_lines.len(),
                mino,
                &mut local_send_line_events,
            );
        }
    }
}

fn clear_line(field: &mut Field, clear_lines: &[i8]) {
    for &clear_y in clear_lines {
        for y in clear_y..(FIELD_MAX_HEIGHT - 1) {
            field.lines[y as usize] = field.lines[(y + 1) as usize];
        }
        field.lines[(FIELD_MAX_HEIGHT - 1) as usize] =
            [FieldBlock::default(); FIELD_WIDTH as usize];
    }
}

fn handle_local_field(
    field: &Field,
    local_field: &mut LocalField,
    clear_line_count: usize,
    mino: &Mino,
    local_send_line_events: &mut EventWriter<LocalSendLinesEvent>,
) {
    // 基本のおじゃま行
    let lines = match clear_line_count {
        2 => 1,
        3 => 2,
        4.. => 4,
        _ => 0,
    };

    // Tスピンの場合は2倍
    let lines = match mino.t_spin {
        TSpin::None | TSpin::Mini => lines,
        TSpin::Full => lines * 2,
    };

    let is_difficult_clear = clear_line_count >= 4 || mino.t_spin != TSpin::None;

    // back-to-backの場合は+1
    let lines = if local_field.can_back_to_back && is_difficult_clear {
        lines + 1
    } else {
        lines
    };

    // LENボーナス
    let lines = match local_field.len {
        0 => lines,
        1..=3 => lines + 1,
        4..=5 => lines + 2,
        6..=7 => lines + 3,
        8..=10 => lines + 4,
        11.. => lines + 5,
    };

    // パーフェクトクリアの場合は+10
    let lines = if field
        .lines
        .iter()
        .all(|line| line.iter().all(|block| block.is_empty()))
    {
        lines + 10
    } else {
        lines
    };

    // フィールドの状態を更新
    if clear_line_count != 0 {
        local_field.can_back_to_back = is_difficult_clear;
        local_field.len += 1;
    } else {
        local_field.len = 0;
    }

    // おじゃま行を送る
    if_chain! {
        if lines > 0;
        if let Some(target_player_id) = local_field.target_player_id;
        then {
            local_send_line_events.send(LocalSendLinesEvent {
                player_id: target_player_id,
                lines,
            })
        }
    }
}
