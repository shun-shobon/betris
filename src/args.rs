use bevy::prelude::*;
use clap::Parser;
use serde::Deserialize;

#[derive(Debug, Clone, Parser, Deserialize, Resource)]
#[clap(name = "Betris")]
pub struct Args {
    #[clap(long, default_value = "ws://localhost:3536")]
    pub matchbox: String,
    #[clap(short, long, default_value = "1")]
    pub players: usize,
}
