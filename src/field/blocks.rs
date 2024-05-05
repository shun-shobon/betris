use super::{block::Block, FIELD_MAX_HEIGHT, FIELD_WIDTH};
use crate::{
    mino::{shape::Shape, Angle, Mino},
    pos,
    position::Position,
};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub struct Blocks([[Block; FIELD_WIDTH as usize]; FIELD_MAX_HEIGHT as usize]);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lines(Vec<u8>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Garbages(Vec<u8>);

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

    pub fn can_place_mino(&self, mino_pos: Position, shape: Shape, angle: Angle) -> bool {
        shape
            .blocks(angle)
            .iter()
            .map(|&pos| pos + mino_pos)
            .all(|pos| self.get(pos).map_or(false, Block::is_empty))
    }

    pub fn place_mino(&mut self, mino: &Mino) {
        for &block_pos in mino.shape.blocks(mino.angle) {
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

    #[allow(clippy::result_unit_err)]
    pub fn add_garbages(&mut self, garbages: &Garbages) -> Result<(), ()> {
        if self.is_gameorver(garbages) {
            return Err(());
        }

        for y in (0..(FIELD_MAX_HEIGHT as usize - 1 - garbages.len())).rev() {
            self.0[y + garbages.len()] = self.0[y];
        }

        for (y, hole_x) in garbages.0.iter().rev().enumerate() {
            for x in 0..(FIELD_WIDTH as usize) {
                self.0[y][x] = if x == *hole_x as usize {
                    Block::Empty
                } else {
                    Block::Garbage
                };
            }
        }

        Ok(())
    }

    fn is_gameorver(&self, garbages: &Garbages) -> bool {
        !self.0[(FIELD_MAX_HEIGHT as usize - garbages.len())..(FIELD_MAX_HEIGHT as usize)]
            .iter()
            .all(|line| line.iter().all(Block::is_empty))
    }

    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|line| line.iter().all(Block::is_empty))
    }

    pub(crate) fn check_pos(pos: Position) -> bool {
        0 <= pos.x && pos.x < FIELD_WIDTH && 0 <= pos.y && pos.y < FIELD_MAX_HEIGHT
    }
}

impl Lines {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

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

pub(crate) fn get_random_x() -> u8 {
    rand::thread_rng().gen_range(0..(FIELD_WIDTH as u8))
}
