use bevy::prelude::*;

pub mod input;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Splash,
    Intro,
    Game,
    EndGame,
}

pub fn despawn_screen<T: Component>(
    to_despawn: Query<Entity, With<T>>, 
    mut commands: Commands
) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}