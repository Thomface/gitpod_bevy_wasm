use bevy::prelude::*;

use super::structs::game::Game;

pub struct DeckPlugin;

const CARD_SIZE: f32 = 100.0;
const CARD_SIZE_RATIO: f32 = 1.5;

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_deck);
        /*app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_deck))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(handle_input));*/
    }
}

fn setup_deck(
    mut commands: Commands,
    mut game: ResMut<Game>,
    /*mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,*/
) {
    game.draw();

    let mut idx = 0;
    for card in game.hand.iter_mut() {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(CARD_SIZE, CARD_SIZE * CARD_SIZE_RATIO)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(100.0 * CARD_SIZE, 0.0, 1.0),
                ..default()
            },
            ..default()
        }).insert(*card);
        idx += 1;
    }
}
/*
// control the game character
fn move_player(
    mut animation_player: Query<&mut AnimationPlayer>,
    model_assets: Res<ModelAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    for (mut transform, mut player) in query.iter_mut() {
        let mut moving = false;
        let mut rotation = 0.0;
        if keyboard_input.pressed(KeyCode::Up) {
            if player.x < BOARD_SIZE as f32 - 1.0 {
                player.x += 0.1;
            }
            rotation = std::f32::consts::FRAC_PI_2;
            moving = true;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            if player.x > 0.0 {
                player.x -= 0.1;
            }
            rotation = -std::f32::consts::FRAC_PI_2;
            moving = true;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            if player.z < BOARD_SIZE as f32 - 1.0 {
                player.z += 0.1;
            }
            rotation = 0.0;
            moving = true;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            if player.z > 0.0 {
                player.z -= 0.1;
            }
            rotation = std::f32::consts::PI;
            moving = true;
        }

        // move on the board
        if let Ok(mut my_animation_player) = animation_player.get_single_mut() {
            if let Some(gltf) = assets_gltf.get(&model_assets.robot) {
                if moving {
                    if !player.walking {
                        my_animation_player
                            .play(gltf.named_animations["Robot_Walking"].clone())
                            .set_speed(2.0)
                            .repeat();
                        player.walking = true;
                    }
                    transform.translation = Vec3::new(player.x, player.y, player.z);
                    transform.rotation = Quat::from_rotation_y(rotation);
                } else {
                    player.walking = false;
                    my_animation_player.play(gltf.named_animations["Robot_Idle"].clone()).repeat();
                }
            }
        }
    }
}
*/