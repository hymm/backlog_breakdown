use bevy::prelude::*;

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameState {
    #[default]
    StartScreen,
    Playing,
    Failed,
}
