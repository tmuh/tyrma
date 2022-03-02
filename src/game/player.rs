use bevy::prelude::*;
use crate::core::{
    input::PlayerMovementEvent,
    input::PlayerDirection,
};
use super::OnInGameScreen;

pub fn player_setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    // TODO: Spawn player sprite.
}

const MOVEMENT_SPEED: f32 = 1.0;

pub fn player_movement(
    time: Res<Time>,
    mut events: EventReader<PlayerMovementEvent>
) {

    for movement_event in events.iter() {
        match movement_event.direction {
            PlayerDirection::Left => {
                // TODO: update direction & position
            }
            PlayerDirection::Right => {
                // TODO: update direction & position
            }
        }
    }
}