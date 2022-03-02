use bevy::prelude::*;
use super::core::{
    despawn_screen, 
    GameState,
    input::ConfirmEvent
};

pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_enter(GameState::Intro)
            .with_system(intro_setup))
        .add_system_set(SystemSet::on_update(GameState::Intro)
            .with_system(intro_update))
        .add_system_set(SystemSet::on_exit(GameState::Intro)
            .with_system(despawn_screen::<OnIntroScreen>));
    }
}

#[derive(Component)]
struct OnIntroScreen;

fn intro_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "Blaa blaa blaa\n\n (Intro text placeholder).",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        .insert(OnIntroScreen);
}

fn intro_update(
    mut events: EventReader<ConfirmEvent>,
    mut game_state: ResMut<State<GameState>>
) {
    for _my_event in events.iter() {
        info!("Confirm key received");
        game_state.set(GameState::Game).unwrap();
    }
}