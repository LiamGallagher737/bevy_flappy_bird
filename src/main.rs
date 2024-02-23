#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{
    prelude::*,
    window::{EnabledButtons, WindowResolution},
};

mod game;
mod game_over;
mod menu;

const PIPE_Z: f32 = 1.0;
const GROUND_Z: f32 = 2.0;
const BIRD_Z: f32 = 3.0;
const UI_Z: f32 = 4.0;

const BIRD_SIZE: Vec2 = Vec2::new(34.0, 24.0);
const PIPE_SIZE: Vec2 = Vec2::new(52.0, 320.0);
const GROUND_WIDTH: f32 = 336.0;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Flappy Bird"),
                        resolution: WindowResolution::new(288.0, 512.0),
                        resizable: false,
                        enabled_buttons: EnabledButtons {
                            maximize: false,
                            minimize: false,
                            close: true,
                        },
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_state::<GameState>()
        .add_systems(Startup, scene_setup)
        .add_plugins((
            game::GamePlugin,
            game_over::GameOverPlugin,
            menu::MenuPlugin,
        ))
        .run();
}

#[derive(Resource)]
struct AudioHandles {
    flap: Handle<AudioSource>,
    hit: Handle<AudioSource>,
    point: Handle<AudioSource>,
}

#[derive(Component)]
struct Scroll;

#[derive(Component)]
struct Ground;

fn scene_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a 2D camera
    commands.spawn(Camera2dBundle::default());

    // Spawn the background sprite
    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/background.png"),
        ..Default::default()
    });

    // Spawn 2 ground sprites so that they can scroll infinitely
    let texture_handle = asset_server.load("sprites/ground.png");
    for i in 0..2 {
        commands.spawn((
            Ground,
            Scroll,
            SpriteBundle {
                texture: texture_handle.clone(),
                transform: Transform::from_xyz(i as f32 * GROUND_WIDTH, -200.0, GROUND_Z),
                ..Default::default()
            },
        ));
    }

    // Load audio files
    commands.insert_resource(AudioHandles {
        flap: asset_server.load("audio/flap.ogg"),
        hit: asset_server.load("audio/hit.ogg"),
        point: asset_server.load("audio/point.ogg"),
    });
}

// Return true if the user has clicked, tapped or pressed the space bar
pub fn has_user_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    touch_input: Res<Touches>,
) -> bool {
    keyboard_input.just_pressed(KeyCode::Space)
        || mouse_button_input.just_pressed(MouseButton::Left)
        || touch_input.any_just_pressed()
}

// Despawn all entities recursively with a given component
pub fn cleanup<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
