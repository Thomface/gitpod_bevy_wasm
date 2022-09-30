use bevy::{prelude::*};

mod plugin;

use plugin::GamePlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Bevy Playground".to_string(),
            #[cfg(target_arch = "wasm32")]
            canvas: Some("#playground".to_string()),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}