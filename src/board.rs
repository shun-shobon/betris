use bevy::prelude::*;

use crate::mino::MinoColor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct Position {
    pub x: i8,
    pub y: i8,
}

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;

#[derive(Component)]
pub struct Board(pub [[Option<MinoColor>; BOARD_WIDTH]; BOARD_HEIGHT]);

#[derive(Component)]
pub struct Block;

impl Default for Board {
    fn default() -> Self {
        let mut board = [[None; BOARD_WIDTH]; BOARD_HEIGHT];

        board[0][0] = Some(MinoColor::I);
        board[0][1] = Some(MinoColor::J);
        board[0][2] = Some(MinoColor::L);
        board[0][3] = Some(MinoColor::O);
        board[1][0] = Some(MinoColor::S);
        board[1][1] = Some(MinoColor::T);
        board[1][2] = Some(MinoColor::Z);

        Self(board)
    }
}
