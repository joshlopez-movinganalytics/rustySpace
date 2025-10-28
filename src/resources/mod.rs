pub mod game_state;

pub use game_state::*;

use bevy::prelude::*;

/// Timer for spawning enemies
#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

