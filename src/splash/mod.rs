use bevy::prelude::*;
use super::core::{
    despawn_screen, 
    GameState, 
    input::AnyKeyEvent
};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Splash)
                .with_system(splash_setup))
            .add_system_set(SystemSet::on_update(GameState::Splash)
                .with_system(splash_input))
            .add_system_set(SystemSet::on_exit(GameState::Splash)
                .with_system(despawn_screen::<OnSplashScreen>));
    }
}

#[derive(Component)]
struct OnSplashScreen;

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("branding/splash.png");
    // Display the logo
    commands
        .spawn_bundle(ImageBundle {
            style: Style {
                // This will center the logo
                margin: Rect::all(Val::Auto),
                // This will set the logo to be 200px wide, and auto adjust its height
                size: Size::new(Val::Px(200.0), Val::Auto),
                ..Default::default()
            },
            image: UiImage(icon),
            ..Default::default()
        })
        .insert(OnSplashScreen);
}

fn splash_input(
    mut events: EventReader<AnyKeyEvent>,
    mut game_state: ResMut<State<GameState>>,
) {
    for _my_event in events.iter() {
        info!("Any key received");
        game_state.set(GameState::Intro).unwrap();
    }
}