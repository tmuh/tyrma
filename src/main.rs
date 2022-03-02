use bevy::{prelude::*};
mod core;
mod splash;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(WindowDescriptor {
            title: "Tyrmä".to_string(),
            width: 1200.,
            height: 1200.,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_state(core::GameState::Splash)
        .add_plugin(core::input::InputPlugin)
        .add_plugin(splash::SplashPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}
