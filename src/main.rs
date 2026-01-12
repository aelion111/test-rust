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
        .add_system(spawn_start_ui.in_schedule(OnEnter(GameState::Start)))
        .add_system(start_game.run_if(in_state(GameState::Start)))

        // ===== PLAYING =====
        .add_system(cleanup_game_entities.in_schedule(OnEnter(GameState::Playing)))
        .add_systems(
            (spawn_paddle, spawn_ball, spawn_bricks)
                .in_schedule(OnEnter(GameState::Playing)),
        )
        .add_system(cleanup_game_entities.in_schedule(OnExit(GameState::Playing)))
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
                .run_if(in_state(GameState::Playing)),
        )

        // ===== GAME OVER =====
        .add_system(spawn_game_over_ui.in_schedule(OnEnter(GameState::GameOver)))
        .add_system(restart_game.run_if(in_state(GameState::GameOver)))

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
