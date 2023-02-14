use super::UI_Z;
use crate::{cleanup, is_input, GameState};
use bevy::prelude::*;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_schedule(OnEnter(GameState::Menu), setup_menu)
            .add_system_to_schedule(OnExit(GameState::Menu), cleanup::<MenuEntity>)
            .add_system(start_playing.in_set(OnUpdate(GameState::Menu)).run_if(is_input));
    }
}

#[derive(Component)]
struct MenuEntity;

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/start.png"),
            transform: Transform::from_xyz(0.0, 80.0, UI_Z),
            ..Default::default()
        },
        MenuEntity,
    ));
}

fn start_playing(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Playing);
}
