use super::{shape::Shape, t_spin::TSpin, Mino};
use crate::{
    field::{
        blocks::{Garbages, Lines},
        local::LocalField,
        timer::DropTimer,
        Field,
    },
    net::{send_garbage, sync_local_field_change, PlayerId, Players, Socket},
    state::GameOverEvent,
};
use bevy::prelude::*;

#[derive(Event)]
pub struct SpawnMinoEvent(pub Shape);

#[derive(Event)]
pub struct SyncFieldChangeEvent {
    pub player_id: PlayerId,
    pub mino: Mino,
    pub clear_lines: Lines,
    pub garbage_lines: Garbages,
}

#[derive(Event)]
pub struct PlaceMinoEvent;

pub fn handle_spawn_mino(
    mut commands: Commands,
    mut events: EventReader<SpawnMinoEvent>,
    mut field_query: Query<(Entity, &Field, &mut DropTimer), With<LocalField>>,
    mut gameover_events: EventWriter<GameOverEvent>,
) {
    let Ok((field_entity, field, mut drop_timer)) = field_query.get_single_mut() else {
        return;
    };

    for SpawnMinoEvent(shape) in events.read() {
        if let Some(mino) = Mino::new(*shape, field) {
            let mino_entity = mino.spawn(&mut commands);
            commands.entity(field_entity).add_child(mino_entity);

            drop_timer.0.reset();
        } else {
            gameover_events.send(GameOverEvent);
        }
    }
}

pub fn handle_sync_field_change(
    mut events: EventReader<SyncFieldChangeEvent>,
    mut field_query: Query<&mut Field>,
) {
    for event in events.read() {
        let Some(mut field) = field_query
            .iter_mut()
            .find(|field| field.player.id == event.player_id)
        else {
            continue;
        };

        field.blocks.place_mino(&event.mino);
        field.blocks.clear_lines(&event.clear_lines);
        let _ = field.blocks.add_garbages(&event.garbage_lines);
    }
}

#[allow(clippy::too_many_arguments)]
pub fn handle_place_mino(
    mut commands: Commands,
    mut events: EventReader<PlaceMinoEvent>,
    mut socket: ResMut<Socket>,
    players: Res<Players>,
    mut field_query: Query<(&mut Field, &mut LocalField)>,
    mino_query: Query<(Entity, &Mino)>,
    mut spawn_mino_events: EventWriter<SpawnMinoEvent>,
    mut gameover_events: EventWriter<GameOverEvent>,
) {
    for _ in events.read() {
        let Ok((mut field, mut local_field)) = field_query.get_single_mut() else {
            continue;
        };
        let Ok((mino_entity, mino)) = mino_query.get_single() else {
            continue;
        };
        commands.entity(mino_entity).despawn_recursive();

        field.blocks.place_mino(mino);

        let clear_lines = field.blocks.get_filled_lines();
        field.blocks.clear_lines(&clear_lines);

        // フィールドの状態を更新
        if !clear_lines.is_empty() {
            local_field.can_back_to_back = is_difficult_clear(&clear_lines, &local_field);
            local_field.combo += 1;
        } else {
            local_field.combo = 0;
        }

        // おじゃま行を送る
        if let Some(target_player_id) = local_field.target_player_id {
            let garbage_amount = get_garbage_amount(&clear_lines, &local_field, &field);
            if garbage_amount != 0 {
                send_garbage(&mut socket, target_player_id, garbage_amount);
            }
        }

        // おじゃま行を受け取る
        let garbage_lines = Garbages::from_amount(local_field.garbage_amount);
        local_field.garbage_amount = 0;
        local_field.is_hold_used = false;
        let is_gameover = field.blocks.add_garbages(&garbage_lines).is_err();

        // フィールドの状態の変更を通知
        sync_local_field_change(&mut socket, &players, *mino, clear_lines, garbage_lines);

        if is_gameover {
            gameover_events.send(GameOverEvent);
        } else {
            spawn_mino_events.send(SpawnMinoEvent(local_field.next_queue.pop()));
        }
    }
}

fn get_garbage_amount(clear_lines: &Lines, local_field: &LocalField, field: &Field) -> u8 {
    if clear_lines.is_empty() {
        return 0;
    }

    // パーフェクトクリアの場合は10固定
    if field.blocks.is_empty() {
        return 10;
    }

    // 基本のおじゃま行数
    let basic = match (clear_lines.len(), local_field.t_spin) {
        (1, TSpin::None) => 0, // Single
        (2, TSpin::None) => 1, // Double
        (3, TSpin::None) => 2, // Triple
        (4, TSpin::None) => 4, // Tetris
        (1, TSpin::Mini) => 0, // T-Spin Mini Single
        (2, TSpin::Mini) => 1, // T-Spin Mini Double
        (1, TSpin::Full) => 2, // T-Spin Single
        (2, TSpin::Full) => 4, // T-Spin Double
        (3, TSpin::Full) => 6, // T-Spin Triple
        _ => unreachable!(),
    };

    // RENボーナス
    let combo_bonus = match local_field.combo {
        0..=1 => 0,
        2..=3 => 1,
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

    basic + combo_bonus + back_to_back_bonus
}

// テトリスやTスピンといった難しいライン消去か
fn is_difficult_clear(clear_lines: &Lines, local_field: &LocalField) -> bool {
    clear_lines.len() == 4 || local_field.t_spin != TSpin::None
}
