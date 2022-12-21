use bevy::prelude::*;

use super::structs::game::Game;

use super::bevy_interact_2d;
#[cfg(feature = "debug")]
use bevy_interact_2d::InteractionDebugPlugin as InteractionPlugin;
#[cfg(not(feature = "debug"))]
use bevy_interact_2d::InteractionPlugin;
use bevy_interact_2d::{
  drag::{DragPlugin, Draggable, Dragged, DropStrategy},
  Group, Interactable, InteractionSource, InteractionState,
};

pub struct DeckPlugin;

const CARD_SIZE: f32 = 100.0;
const CARD_SIZE_RATIO: f32 = 1.5;

pub const CARD_IN_HAND: u8 = 0;
pub const ENEMI: u8 = 1;

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
        }).insert(Interactable {
            groups: vec![Group(CARD_IN_HAND)],
            bounding_box: (Vec2::new(-12., -12.), Vec2::new(12., 12.)),
            ..Default::default()
        })        
        .insert(Draggable {
            groups: vec![Group(CARD_IN_HAND)],
            hook: None,
            drop_strategy: DropStrategy::Reset
        });
        //.insert(*card);
    }
}