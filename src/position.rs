use crate::field::{block::BLOCK_SIZE, FIELD_HEIGHT, FIELD_WIDTH};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    pub x: i8,
    pub y: i8,
}

#[macro_export]
macro_rules! pos {
    ($(($x:expr, $y:expr)),*) => {
        [$(pos!($x, $y)),*]
    };
    ($x:expr, $y:expr $(,)?) => {
        $crate::position::Position::new($x, $y)
    };
}

impl Position {
    pub fn translation(self) -> Vec3 {
        Vec3::new(
            (self.x as f32 - FIELD_WIDTH as f32 / 2.) * BLOCK_SIZE,
            (self.y as f32 - FIELD_HEIGHT as f32 / 2.) * BLOCK_SIZE,
            0.0,
        )
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Position {
    pub const fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }
}
