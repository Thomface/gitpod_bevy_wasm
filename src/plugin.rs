use bevy::{prelude::*, render::camera::ScalingMode};

mod deck;
mod structs;

use deck::DeckPlugin;
use structs::game::Game;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Game::default())
            // External plugins
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
        });

    // @DEV texture hot reloading
    asset_server.watch_for_changes().unwrap();
}