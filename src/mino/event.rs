use super::{t_spin::TSpin, Mino};
use crate::{
    field::{local::LocalField, Field, FilledLines},
    net::{send_garbage, Socket},
};
use bevy::prelude::*;
use if_chain::if_chain;

#[derive(Event)]
pub struct SpawnMinoEvent;

#[derive(Event)]
pub struct PlaceMinoEvent(pub Mino);

pub fn handle_spawn_mino(
    mut commands: Commands,
    mut events: EventReader<SpawnMinoEvent>,
    mut field_query: Query<(Entity, &mut LocalField)>,
) {
    for _ in events.iter() {
        let Ok((field_entity, mut local_field)) = field_query.get_single_mut() else { continue; };

        let mino_shape = local_field.random_bag.next().unwrap();

        let mino_entity = Mino::new(mino_shape).spawn(&mut commands);
        commands.entity(field_entity).add_child(mino_entity);
    }
}

pub fn handle_place_mino(
    mut events: EventReader<PlaceMinoEvent>,
    mut field_query: Query<(&mut Field, &mut LocalField)>,
    mut socket: ResMut<Socket>,
) {
    for PlaceMinoEvent(mino) in events.iter() {
        let Ok((mut field, mut local_field)) = field_query.get_single_mut() else { continue; };

        field.blocks.place_mino(mino);

        let filled_lines = field.blocks.get_filled_lines();
        field.blocks.clear_lines(&filled_lines);

        let garbage_lines = get_garbage_lines(&filled_lines, &local_field, &field);

        // フィールドの状態を更新
        if !filled_lines.is_empty() {
            local_field.can_back_to_back = is_difficult_clear(&filled_lines, &local_field);
            local_field.len += 1;
        } else {
            local_field.len = 0;
        }

        // おじゃま行を送る
        if_chain! {
            if garbage_lines != 0;
            if let Some(target_player_id) = local_field.target_player_id;
            then {
                send_garbage(&mut socket, target_player_id, garbage_lines);
            }
        }

        // おじゃま行を受け取る

        // フィールドの状態の変更を通知
    }
}

fn get_garbage_lines(clear_lines: &FilledLines, local_field: &LocalField, field: &Field) -> u8 {
    if clear_lines.is_empty() {
        return 0;
    }

    // 基本のおじゃま行数
    let basic = match (clear_lines.len(), local_field.t_spin) {
        (1, TSpin::None) => 0,                    // Single
        (2, TSpin::None) => 1,                    // Double
        (3, TSpin::None) => 2,                    // Triple
        (4, TSpin::None) => 4,                    // Tetris
        (1, TSpin::Mini) | (2, TSpin::Mini) => 0, // T-Spin Mini
        (1, TSpin::Full) => 2,                    // T-Spin Single
        (2, TSpin::Full) => 4,                    // T-Spin Double
        (3, TSpin::Full) => 6,                    // T-Spin Triple
        _ => unreachable!(),
    };

    // LENボーナス
    let len_bonus = match local_field.len {
        0 => 0,
        1..=3 => 1,
        4..=5 => 2,
        6..=7 => 3,
        8..=10 => 4,
        11.. => 5,
    };

    // Back to Backの場合は+1
    let back_to_back_bonus =
        if local_field.can_back_to_back && is_difficult_clear(clear_lines, local_field) {
            1
        } else {
            0
        };

    // パーフェクトクリアの場合は+10
    let perfect_clear_bonus = if field.blocks.is_empty() { 10 } else { 0 };

    basic + len_bonus + back_to_back_bonus + perfect_clear_bonus
}

// テトリスやTスピンといった難しいライン消去か
fn is_difficult_clear(clear_lines: &FilledLines, local_field: &LocalField) -> bool {
    clear_lines.len() == 4 || local_field.t_spin != TSpin::None
}
