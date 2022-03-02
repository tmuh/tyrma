use bevy::{
    input::{
        keyboard::KeyCode, 
        Input, 
        keyboard::KeyboardInput,
        mouse::MouseButtonInput
    },
    prelude::*
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<AnyKeyEvent>()
            .add_event::<PlayerMovementEvent>()
            .add_system(keyboard_input)
            .add_system(keyboard_anykey_input)
            .add_system(mouse_input);
    }
}

pub struct AnyKeyEvent;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayerDirection {
    Left,
    Right,
}

pub struct PlayerMovementEvent {
    direction: PlayerDirection,
}

fn keyboard_anykey_input(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut any_key_events: EventWriter<AnyKeyEvent>,
) {
    for _event in keyboard_input_events.iter() {
        any_key_events.send(AnyKeyEvent);
    }
}

fn keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_movement_events: EventWriter<PlayerMovementEvent>,
) {
    if keyboard_input.pressed(KeyCode::A) {
        player_movement_events.send(PlayerMovementEvent {
            direction: PlayerDirection::Left
        })
    }

    if keyboard_input.pressed(KeyCode::D) {
        player_movement_events.send(PlayerMovementEvent {
            direction: PlayerDirection::Right
        })
    }
}

fn mouse_input(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut any_key_events: EventWriter<AnyKeyEvent>,
) {
    for event in mouse_button_input_events.iter() {
        any_key_events.send(AnyKeyEvent);
    }
}