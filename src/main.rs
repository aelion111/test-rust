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
        .add_system(paddle_movement.run_if(in_state(GameState::Playing)))
        .add_system(ball_movement.run_if(in_state(GameState::Playing)))
        .add_system(confine_paddle.run_if(in_state(GameState::Playing)))
        .add_system(confine_ball.run_if(in_state(GameState::Playing)))
        .add_system(update_ball_direction.run_if(in_state(GameState::Playing)))
        .add_system(ball_brick_collision.run_if(in_state(GameState::Playing)))
        .add_system(ball_paddle_collision.run_if(in_state(GameState::Playing)))
        .add_system(power_up_fall.run_if(in_state(GameState::Playing)))
        .add_system(paddle_collect_power_up.run_if(in_state(GameState::Playing)))
        .add_system(check_game_over.run_if(in_state(GameState::Playing)))
        .add_system(update_score.run_if(in_state(GameState::Playing)))
        .add_system(handle_game_over.run_if(in_state(GameState::Playing)))

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
