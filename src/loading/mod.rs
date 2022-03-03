use bevy::{prelude::*};
use bevy_loading::prelude::*;
use crate::core::{
    despawn_screen, 
    GameState,
};

pub struct GameAssetLoadPlugin;

impl Plugin for GameAssetLoadPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(LoadingPlugin {
                loading_state: GameState::Loading,
                next_state: GameState::Intro,
            })
            .add_system_set(SystemSet::on_enter(GameState::Loading)
                .with_system(loading_setup))
            .add_system_set(
                SystemSet::on_update(GameState::Loading)
                    .with_system(load_game_assets)
            )
            .add_system_set(SystemSet::on_exit(GameState::Loading)
                .with_system(despawn_screen::<OnLoadingScreen>));
    }
}

#[derive(Component)]
struct OnLoadingScreen;

pub struct PlayerAssets {
    pub player_left: Handle<Image>,
    pub player_right: Handle<Image>,
}

fn load_game_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
) {
    info!("Loading game assets...");
    let player_left = asset_server.load("sprites/player_left.png");
    let player_right = asset_server.load("sprites/player_right.png");

    loading.add(&player_left);
    loading.add(&player_right);

    commands.insert_resource(PlayerAssets {
        player_left,
        player_right,
    });
}

fn loading_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
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
                "Loading...",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 24.0,
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
        .insert(OnLoadingScreen);
}