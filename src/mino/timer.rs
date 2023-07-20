use bevy::prelude::*;

#[derive(Component)]
pub struct DropTimer(pub Timer);

#[derive(Component)]
pub struct LockDownTimer(pub Timer);
