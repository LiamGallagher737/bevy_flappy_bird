use super::{bird::Bird, ApproachingPipe, Pipe, PipeSpawnTimer, PlayState, Score};
use super::{GAP_HEIGHT, PIPE_SPAWN_OFFSET};
use crate::game_over::DespawnOnReset;
use crate::{Scroll, BIRD_SIZE, PIPE_SIZE, PIPE_Z};
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::*;
use rand::Rng;

// Spawn a new pipe pair
pub(super) fn spawn_pipe(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut timer: ResMut<PipeSpawnTimer>,
) {
    timer.0.tick(time.delta());

    if !timer.0.finished() {
        return;
    }

    let mut rng = rand::thread_rng();
    let y = rng.gen_range(-50.0..50.0);

    let texture = asset_server.load("sprites/pipe.png");

    commands.spawn((
        Pipe,
        ApproachingPipe,
        Scroll,
        DespawnOnReset,
        SpriteBundle {
            texture: texture.clone(),
            transform: Transform::from_xyz(PIPE_SPAWN_OFFSET, y - 160.0, PIPE_Z),
            ..Default::default()
        },
    ));

    commands.spawn((
        Pipe,
        Scroll,
        DespawnOnReset,
        SpriteBundle {
            texture,
            transform: Transform::from_xyz(PIPE_SPAWN_OFFSET, y + 160.0 + GAP_HEIGHT, PIPE_Z),
            sprite: Sprite {
                flip_y: true,
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}

// Despawn pipes that have moved off screen
pub(super) fn despawn_pipe(mut commands: Commands, query: Query<(Entity, &Transform), With<Pipe>>) {
    for (entity, transform) in &query {
        if transform.translation.x < -PIPE_SPAWN_OFFSET {
            commands.entity(entity).despawn();
        }
    }
}

// Check if the bird has progressed passed a pipe and add to the score
pub(super) fn check_passed_pipe(
    mut commands: Commands,
    mut score: ResMut<Score>,
    pipes: Query<(Entity, &Transform), With<ApproachingPipe>>,
    bird: Query<&Transform, With<Bird>>,
) {
    let bird = bird.single();
    for (entity, pipe) in &pipes {
        if pipe.translation.x + PIPE_SIZE.x / 2.0 < bird.translation.x - BIRD_SIZE.x / 2.0 {
            commands.entity(entity).remove::<ApproachingPipe>();
            score.0 += 1;
            break;
        }
    }
}

// Check if the bird has collided with a pipe and end the game
pub(super) fn check_pipe_collision(
    mut play_state: ResMut<NextState<PlayState>>,
    bird: Query<&Transform, With<Bird>>,
    pipes: Query<&Transform, With<Pipe>>,
) {
    let bird = bird.single();
    for pipe in &pipes {
        let collision = Aabb2d::new(bird.translation.xy(), BIRD_SIZE / 2.00)
            .intersects(&Aabb2d::new(pipe.translation.xy(), PIPE_SIZE / 2.0));
        if collision {
            play_state.set(PlayState::HitPipe);
        }
    }
}
