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
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.50, 0.25, 0.75),
            custom_size: Some(Vec2::new(CARD_SIZE, CARD_SIZE * CARD_SIZE_RATIO)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(-1.0, 0.0, 1.0),
            ..default()
        },
        ..default()
    });//TODO create a draw component .insert(*card);

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.50, 0.25, 0.75),
            custom_size: Some(Vec2::new(CARD_SIZE, CARD_SIZE * CARD_SIZE_RATIO)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(3.0, 1.0, 2.0),
            ..default()
        },
        ..default()
    });//TODO create a discard component .insert(*card);

    game.draw();
    game.draw();

    for (i, card) in game.hand.iter().enumerate() {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.50, 0.25, 0.75),
                custom_size: Some(Vec2::new(CARD_SIZE, CARD_SIZE * CARD_SIZE_RATIO)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(300.0, 0.0, 0.0) + Vec3::new(i as f32 * CARD_SIZE, 0.0, 1.0),
                ..default()
            },
            ..default()
        }).insert(*card);
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