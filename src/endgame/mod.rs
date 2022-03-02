use bevy::prelude::*;
use super::core::{
    despawn_screen, 
    GameState,
    input::ConfirmEvent
};

pub struct EndGamePlugin;

impl Plugin for EndGamePlugin {
    fn build(&self, app: &mut App) {

    }
}