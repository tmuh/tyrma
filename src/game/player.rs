use bevy::prelude::*;
use crate::core::{
    input::PlayerMovementEvent,
    input::PlayerDirection,
};
use super::OnInGameScreen;
use crate::loading::PlayerAssets;

pub fn player_setup(
    mut commands: Commands, 
    player_assets: Res<PlayerAssets>
) {
    // TODO: Spawn player sprite.
    commands
        .spawn_bundle(SpriteBundle {
            texture: player_assets.player_left.clone(),
            ..Default::default()
        });
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