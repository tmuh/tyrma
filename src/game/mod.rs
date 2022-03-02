use bevy::prelude::*;
use super::core::{
    despawn_screen, 
    GameState,
};

mod player;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<HourChangeEvent>()
            .init_resource::<GameTime>()
            .add_system_set(SystemSet::on_enter(GameState::Game)
                .with_system(ingame_setup)
                .with_system(player::player_setup))
            .add_system_set(SystemSet::on_update(GameState::Game)
                .with_system(update_game_time)
                .with_system(player::player_movement))
            .add_system_set(SystemSet::on_exit(GameState::Game)
                .with_system(despawn_screen::<OnInGameScreen>));
    }
}

#[derive(Component)]
struct OnInGameScreen;

fn ingame_setup(mut commands: Commands, asset_server: Res<AssetServer>) {

}

const SECONDS_IN_GAME_HOUR: f32 = 60.;

struct GameTime {
    pub first_hour: u32,
    pub current_hour: u32,
    pub time_since_last_hour: f32
}

impl GameTime {
    pub fn new() -> Self {
        Self {
            first_hour: 1,
            current_hour: 1,
            time_since_last_hour: 0.,
        }
    }
}

impl Default for GameTime {
    fn default() -> Self {
        Self::new()
    }
}

struct HourChangeEvent {
    current_hour: u32,
}

fn update_game_time(
    time: Res<Time>,
    mut game_time: ResMut<GameTime>,
    mut hour_change_event: EventWriter<HourChangeEvent>
) {
    let updated_time = time.delta_seconds() + game_time.time_since_last_hour;
    if updated_time >= SECONDS_IN_GAME_HOUR {
        game_time.current_hour += 1;
        game_time.time_since_last_hour = 0.;
        hour_change_event.send(HourChangeEvent {
            current_hour: game_time.current_hour
        });
    } else {
        game_time.time_since_last_hour = updated_time;
    }

    info!(game_time.current_hour);
}