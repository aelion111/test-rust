use bevy::prelude::*;

mod components;
mod systems;
mod resources;
mod events;

use systems::*;
use resources::*;
use events::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .init_resource::<Score>()
        .add_event::<GameOver>()
        .add_startup_system(spawn_camera)

        // ===== START =====
        .add_system(spawn_start_ui.on_enter(GameState::Start))
        .add_system(start_game.on_update(GameState::Start))

        // ===== PLAYING =====
        .add_system(cleanup_game_entities.on_enter(GameState::Playing))
        .add_systems(
            (spawn_paddle, spawn_ball, spawn_bricks)
                .on_enter(GameState::Playing),
        )
        .add_system(cleanup_game_entities.on_exit(GameState::Playing))
        .add_systems(
            (
                paddle_movement,
                ball_movement,
                confine_paddle,
                confine_ball,
                update_ball_direction,
                ball_brick_collision,
                ball_paddle_collision,
                power_up_fall,
                paddle_collect_power_up,
                check_game_over,
                update_score,
                handle_game_over,
            )
                .on_update(GameState::Playing),
        )

        // ===== GAME OVER =====
        .add_system(spawn_game_over_ui.on_enter(GameState::GameOver))
        .add_system(restart_game.on_update(GameState::GameOver))

        // ===== GLOBAL =====
        .add_system(exit_game)
        .run();
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Start,
    Playing,
    GameOver,
}
