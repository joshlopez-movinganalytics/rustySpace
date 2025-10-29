use bevy::prelude::*;

/// Game state
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    Paused,
    Upgrade,
    GameOver,
}

