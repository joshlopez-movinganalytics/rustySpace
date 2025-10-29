pub mod game_state;
pub mod galaxy;

pub use game_state::*;
pub use galaxy::*;

use bevy::prelude::*;

/// Timer for spawning enemies
#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

