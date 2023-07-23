use crate::field::{FIELD_HEIGHT, FIELD_WIDTH};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    pub x: i8,
    pub y: i8,
}

impl Position {
    pub fn translation(self, block_size: f32) -> Vec3 {
        Vec3::new(
            (self.x as f32 - FIELD_WIDTH as f32 / 2.) * block_size + block_size / 2.,
            (self.y as f32 - FIELD_HEIGHT as f32 / 2.) * block_size + block_size / 2.,
            0.,
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
