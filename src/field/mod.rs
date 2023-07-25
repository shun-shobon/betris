pub mod block;
pub mod local;
pub mod random;
pub mod timer;

use self::{
    block::{Block, BLOCK_SIZE},
    local::{GarbageWarningBar, LocalFieldBundle},
};
use crate::{mino::Mino, net::PlayerId, pos, position::Position};
use bevy::prelude::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

pub const FIELD_WIDTH: i8 = 10;
pub const FIELD_HEIGHT: i8 = 20;
// この値よりもブロックがせり上がった場合はゲームオーバー
pub const FIELD_MAX_HEIGHT: i8 = FIELD_HEIGHT + 20;

const FIELD_GRID_WIDTH: f32 = 1.;

#[derive(Component)]
pub struct Field {
    pub player_id: PlayerId,
    pub blocks: Blocks,
}

impl Field {
    pub fn new(player_id: PlayerId) -> Self {
        Self {
            player_id,
            blocks: Blocks::default(),
        }
    }

    pub fn spawn(self, commands: &mut Commands, is_local_field: bool, translation: Vec3) -> Entity {
        let mut field_commands = commands.spawn((
            SpatialBundle::from_transform(Transform::from_translation(translation)),
            self,
        ));

        if is_local_field {
            field_commands
                .insert(LocalFieldBundle::default())
                .with_children(|parent| {
                    spawn_grid(parent);
                    GarbageWarningBar::spawn(parent);
                })
                .id()
        } else {
            field_commands.with_children(spawn_grid).id()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Blocks([[Block; FIELD_WIDTH as usize]; FIELD_MAX_HEIGHT as usize]);

impl Default for Blocks {
    fn default() -> Self {
        Self([[Block::default(); FIELD_WIDTH as usize]; FIELD_MAX_HEIGHT as usize])
    }
}

#[allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
)]
impl Blocks {
    pub fn get(&self, pos: Position) -> Option<&Block> {
        if Self::check_pos(pos) {
            self.0.get(pos.y as usize)?.get(pos.x as usize)
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, pos: Position) -> Option<&mut Block> {
        if Self::check_pos(pos) {
            self.0.get_mut(pos.y as usize)?.get_mut(pos.x as usize)
        } else {
            None
        }
    }

    pub fn place_mino(&mut self, mino: &Mino) {
        for &block_pos in mino.shape.blocks(mino.angle).iter() {
            let pos = block_pos + mino.pos;

            let block = self.get_mut(pos).unwrap();
            *block = mino.shape.into();
        }
    }

    pub fn indexed_iter(&self) -> impl Iterator<Item = (Position, &Block)> {
        self.0.iter().enumerate().flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(move |(x, block)| (pos!(x as i8, y as i8), block))
        })
    }

    pub fn get_filled_lines(&self) -> Lines {
        let full_filled_lines = self
            .0
            .iter()
            .enumerate()
            .filter(|(_, line)| line.iter().all(Block::is_filled))
            .map(|(y, _)| y as u8)
            .rev()
            .collect::<Vec<_>>();

        Lines(full_filled_lines)
    }

    pub fn clear_lines(&mut self, full_filled_lines: &Lines) {
        for &clear_y in &full_filled_lines.0 {
            for y in clear_y..(FIELD_MAX_HEIGHT as u8 - 1) {
                self.0[y as usize] = self.0[(y + 1) as usize];
            }
            self.0[(FIELD_MAX_HEIGHT - 1) as usize] = [Block::default(); FIELD_WIDTH as usize];
        }
    }

    pub fn add_garbages(&mut self, _garbage_lines: &Garbages) {
        // TODO: おじゃまラインの実装
    }

    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|line| line.iter().all(Block::is_empty))
    }

    fn check_pos(pos: Position) -> bool {
        0 <= pos.x && pos.x < FIELD_WIDTH && 0 <= pos.y && pos.y < FIELD_MAX_HEIGHT
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lines(Vec<u8>);

impl Lines {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Garbages(Vec<u8>);

impl Garbages {
    pub fn from_amount(amount: u8) -> Self {
        // 一度のおじゃま送信では70%の確率で同じ列に穴が出来る
        let vec = (0..amount)
            .scan(None, |prev, _| match *prev {
                None => {
                    *prev = Some(get_random_x());
                    *prev
                }
                Some(x) => {
                    if rand::thread_rng().gen_bool(0.7) {
                        Some(x)
                    } else {
                        *prev = Some(get_random_x());
                        *prev
                    }
                }
            })
            .collect();

        Self(vec)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

fn get_random_x() -> u8 {
    rand::thread_rng().gen_range(0..(FIELD_WIDTH as u8))
}

fn spawn_grid(parent: &mut ChildBuilder) {
    let width = FIELD_WIDTH as f32 * BLOCK_SIZE;
    let height = FIELD_HEIGHT as f32 * BLOCK_SIZE;

    for y in 0..=FIELD_HEIGHT {
        parent.spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., -(y as f32 - FIELD_HEIGHT as f32 / 2.) * BLOCK_SIZE, 0.),
                ..default()
            },
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(width, FIELD_GRID_WIDTH)),
                ..default()
            },
            ..default()
        });
    }

    for x in 0..=FIELD_WIDTH {
        parent.spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new((x as f32 - FIELD_WIDTH as f32 / 2.) * BLOCK_SIZE, 0., 0.),
                ..default()
            },
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(FIELD_GRID_WIDTH, height)),
                ..default()
            },
            ..default()
        });
    }
}
