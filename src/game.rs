use super::BIRD_Z;
use crate::BIRD_SIZE;
use crate::{game_over::DespawnOnReset, is_input, GameState, Ground, Scroll, GROUND_WIDTH};
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
        app.add_state::<PlayState>()
            .init_resource::<Score>()
            .insert_resource(PipeSpawnTimer(Timer::from_seconds(
                PIPE_SPAWN_TIME,
                TimerMode::Repeating,
            )))
            .add_system_to_schedule(OnEnter(GameState::Playing), game_setup)
            .add_systems_to_schedule(
                OnExit(GameState::Playing),
                (hit_sound, reset_score, reset_timer),
            )
            .add_system_to_schedule(OnEnter(PlayState::HitPipe), hit_sound)
            .add_systems(
                (
                    // Bird
                    bird::fall,
                    bird::move_bird,
                    bird::animate_bird.run_if(state_equals(PlayState::Normal)),
                    bird::jump
                        .run_if(is_input)
                        .run_if(state_equals(PlayState::Normal)),
                    // Pipes
                    pipes::check_passed_pipe.run_if(state_equals(PlayState::Normal)),
                    pipes::check_pipe_collision.run_if(state_equals(PlayState::Normal)),
                    pipes::spawn_pipe.run_if(state_equals(PlayState::Normal)),
                    pipes::despawn_pipe.run_if(state_equals(PlayState::Normal)),
                    // Sound
                    point_sound.run_if(|s: Res<Score>| s.is_changed()),
                    jump_sound
                        .run_if(is_input)
                        .run_if(state_equals(PlayState::Normal)),
                    // Other
                    check_death,
                    update_score_text.run_if(state_equals(PlayState::Normal)),
                    scroll.run_if(state_equals(PlayState::Normal)),
                    reuse_ground.run_if(state_equals(PlayState::Normal)),
                )
                    .in_set(OnUpdate(GameState::Playing)),
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
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    // Load the bird sprite sheet and create a texture atlas from it
    let bird_texture = asset_server.load("sprites/bird.png");
    let texture_atlas = texture_atlases.add(TextureAtlas::from_grid(
        bird_texture,
        BIRD_SIZE,
        4,
        1,
        None,
        None,
    ));

    // Spawn the bird
    commands.spawn((
        Bird::default(),
        DespawnOnReset,
        SpriteSheetBundle {
            texture_atlas,
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
                    size: Size::all(Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|p| {
            p.spawn((
                ScoreText,
                TextBundle::from_section(
                    "0",
                    TextStyle {
                        font: asset_server.load("fonts/flappybird.ttf"),
                        font_size: 80.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::Center),
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

fn jump_sound(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load("audio/flap.ogg"));
}

fn hit_sound(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load("audio/hit.ogg"));
}

fn point_sound(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load("audio/point.ogg"));
}
