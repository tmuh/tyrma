use bevy::{prelude::*};

mod core;
mod splash;
mod loading;
mod intro;
mod game;
mod endgame;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(WindowDescriptor {
            title: "Tyrmä".to_string(),
            width: 1024.,
            height: 768.,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_state(core::GameState::Splash)
        .add_plugin(core::input::InputPlugin)
        .add_plugin(splash::SplashPlugin)
        .add_plugin(loading::GameAssetLoadPlugin)
        .add_plugin(intro::IntroPlugin)
        .add_plugin(game::InGamePlugin)
        .add_plugin(endgame::EndGamePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
