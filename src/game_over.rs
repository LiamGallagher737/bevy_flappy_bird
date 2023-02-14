use crate::{cleanup, is_input, GameState, UI_Z};
use bevy::prelude::*;

pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_schedule(OnEnter(GameState::GameOver), setup_game_over)
            .add_system(
                goto_menu
                    .in_set(OnUpdate(GameState::GameOver))
                    .run_if(is_input),
            )
            .add_system_to_schedule(OnExit(GameState::GameOver), cleanup::<DespawnOnReset>);
    }
}

#[derive(Component, Default)]
pub struct DespawnOnReset;

fn setup_game_over(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/gameover.png"),
            transform: Transform::from_xyz(0.0, 80.0, UI_Z),
            ..Default::default()
        },
        DespawnOnReset,
    ));
}

fn goto_menu(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Menu);
}
