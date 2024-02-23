use super::BIRD_Z;
use crate::{game_over::DespawnOnReset, has_user_input, GameState, Ground, Scroll, GROUND_WIDTH};
use crate::{AudioHandles, BIRD_SIZE};
use bevy::prelude::*;
use bird::Bird;

mod bird;
mod pipes;

const SCROLL_SPEED: f32 = 50.0;
const JUMP_AMOUNT: f32 = 1.5;
const FALL_SPEED: f32 = 5.0;
const FALL_VELOCITY_LIMIT: f32 = -2.0;
const MOVE_SPEED: f32 = 200.0;
const DEATH_HEIGHT: f32 = -125.0;
const PIPE_SPAWN_OFFSET: f32 = 180.0;
const PIPE_SPAWN_TIME: f32 = 4.0;
const GAP_HEIGHT: f32 = 100.0;
const BIRD_ANIMATION_SPEED: f32 = 10.0;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum PlayState {
    #[default]
    Normal,
    HitPipe,
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayState>()
            .init_resource::<Score>()
            .insert_resource(PipeSpawnTimer(Timer::from_seconds(
                PIPE_SPAWN_TIME,
                TimerMode::Repeating,
            )))
            .add_systems(OnEnter(GameState::Playing), game_setup)
            .add_systems(
                OnExit(GameState::Playing),
                (hit_sound, reset_score, reset_timer),
            )
            .add_systems(OnEnter(PlayState::HitPipe), hit_sound)
            .add_systems(
                Update,
                (
                    // Bird
                    bird::animate_bird,
                    bird::jump.run_if(has_user_input),
                    // Pipes
                    pipes::check_passed_pipe,
                    pipes::check_pipe_collision,
                    pipes::spawn_pipe,
                    pipes::despawn_pipe,
                    // Sound
                    flap_sound.run_if(has_user_input),
                    point_sound.run_if(resource_changed::<Score>),
                    // Other
                    update_score_text,
                    scroll,
                    reuse_ground,
                )
                    .run_if(in_state(GameState::Playing).and_then(in_state(PlayState::Normal))),
            )
            .add_systems(
                Update,
                // These will cotinue running after a pipe is hit
                (
                    // Bird
                    bird::fall,
                    bird::move_bird,
                    // Other
                    check_death,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Resource, Default)]
pub struct Score(usize);

#[derive(Component)]
struct ScoreText;

#[derive(Resource)]
struct PipeSpawnTimer(Timer);

#[derive(Component)]
struct Pipe;

#[derive(Component)]
struct ApproachingPipe;

fn game_setup(
    mut commands: Commands,
    mut play_state: ResMut<NextState<PlayState>>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    // Load the bird sprite sheet and create a texture atlas from it
    let atlas_layout =
        texture_atlases.add(TextureAtlasLayout::from_grid(BIRD_SIZE, 4, 1, None, None));

    // Spawn the bird
    commands.spawn((
        Bird::default(),
        DespawnOnReset,
        SpriteSheetBundle {
            atlas: TextureAtlas {
                layout: atlas_layout,
                index: 0,
            },
            texture: asset_server.load("sprites/bird.png"),
            transform: Transform::from_xyz(0.0, 0.0, BIRD_Z),
            ..Default::default()
        },
    ));

    // Spawn the score UI
    commands
        .spawn((
            DespawnOnReset,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|node| {
            node.spawn((
                ScoreText,
                TextBundle::from_section(
                    "0",
                    TextStyle {
                        font: asset_server.load("fonts/flappybird.ttf"),
                        font_size: 80.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_justify(JustifyText::Center),
            ));
        });

    // Make sure the PlayState is set to Normal
    play_state.set(PlayState::Normal);
}

// Set the score text to display the current score
fn update_score_text(mut query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    if !score.is_changed() {
        return;
    }

    for mut text in &mut query {
        text.sections[0].value = score.0.to_string();
    }
}

// Scroll all entities with the Scroll component
fn scroll(mut query: Query<&mut Transform, With<Scroll>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.translation.x -= SCROLL_SPEED * time.delta_seconds();
    }
}

// If a ground entity is off screen, move it back to te start
fn reuse_ground(mut query: Query<&mut Transform, With<Ground>>) {
    for mut transform in &mut query {
        if transform.translation.x < -GROUND_WIDTH {
            transform.translation.x += GROUND_WIDTH * 2.0;
        }
    }
}

// End the game if the bird is below the death height
fn check_death(bird: Query<&Transform, With<Bird>>, mut state: ResMut<NextState<GameState>>) {
    for bird in &bird {
        if bird.translation.y < DEATH_HEIGHT {
            state.set(GameState::GameOver);
        }
    }
}

fn reset_score(mut score: ResMut<Score>) {
    score.0 = 0;
}

fn reset_timer(mut timer: ResMut<PipeSpawnTimer>) {
    timer.0.reset();
}

fn flap_sound(audio_handles: Res<AudioHandles>, mut commands: Commands) {
    commands.spawn(AudioBundle {
        source: audio_handles.flap.clone(),
        ..default()
    });
}

fn hit_sound(audio_handles: Res<AudioHandles>, mut commands: Commands) {
    commands.spawn(AudioBundle {
        source: audio_handles.hit.clone(),
        ..default()
    });
}

fn point_sound(audio_handles: Res<AudioHandles>, mut commands: Commands) {
    commands.spawn(AudioBundle {
        source: audio_handles.point.clone(),
        ..default()
    });
}
