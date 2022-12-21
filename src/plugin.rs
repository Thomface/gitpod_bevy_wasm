use bevy::{prelude::*, render::camera::ScalingMode};

mod bevy_interact_2d;
#[cfg(feature = "debug")]
use bevy_interact_2d::InteractionDebugPlugin as InteractionPlugin;
#[cfg(not(feature = "debug"))]
use bevy_interact_2d::InteractionPlugin;
use bevy_interact_2d::{
  drag::{DragPlugin},
  Group, InteractionSource
};

mod deck;
use deck::DeckPlugin;

mod structs;
use structs::game::Game;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Game::default())
            // External plugins
            .add_plugin(InteractionPlugin)
            .add_plugin(DragPlugin)
            // Game plugins
            .add_plugin(DeckPlugin)
            .add_startup_system(setup);
            // Main systems
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // camera
    commands
        .spawn_bundle(Camera2dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::Auto {
                    min_width: 800.,
                    min_height: 600.,
                },
                scale: 2.0,
                ..default()
            },
            ..default()
        })
        .insert(InteractionSource {
            groups: vec![Group(deck::CARD_IN_HAND), Group(deck::ENEMI)],
            ..Default::default()
        });

    // @DEV texture hot reloading
    asset_server.watch_for_changes().unwrap();
}