use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;
use bevy::app::AppExit;

use crate::components::*;
use crate::resources::*;
use crate::events::*;
use crate::GameState;

// ============================================================================
// CONSTANTS
// ============================================================================

const PADDLE_SPEED: f32 = 500.0;
const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 30.0);
const BALL_SPEED: f32 = 250.0;
const BALL_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const BRICK_SIZE: Vec2 = Vec2::new(80.0, 30.0);
const POWER_UP_DROP_CHANCE: f32 = 0.9;
const POWER_UP_SPEED: f32 = 150.0;

// ============================================================================
// COMPONENTS
// ============================================================================

#[derive(Component)]
pub struct PowerUp {}

// ============================================================================
// CAMERA
// ============================================================================

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// ============================================================================
// SPAWNING SYSTEMS
// ============================================================================

pub fn spawn_paddle(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                0.0,
                -window.height() / 2.0 + 50.0,
                0.0,
            ),
            texture: assets_server.load("sprites/paddleBlu.png"),
            ..default()
        },
        Paddle {},
    ));
}

pub fn spawn_ball(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                0.0,
                -window.height() / 2.0 + 80.0,
                0.0,
            ),
            texture: assets_server.load("sprites/ballBlue.png"),
            ..default()
        },
        Ball {
            direction: Vec2::new(random::<f32>(), 1.0).normalize(),
        },
    ));
}

pub fn spawn_bricks(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let rows = 10;
    let cols = window.width() as usize / (BRICK_SIZE.x as usize + 10);

    let start_x = window.width() / -2.0 + BRICK_SIZE.x / 2.0 + 10.0;
    let start_y = window.height() / 2.0 - BRICK_SIZE.y / 2.0 - 10.0;

    for row in 0..rows {
        for col in 0..cols {
            let hp = if row < 4 { 2 } else { 1 };
            let texture = if hp == 2 {
                assets_server.load("sprites/element_yellow_rectangle.png")
            } else {
                assets_server.load("sprites/element_green_rectangle.png")
            };

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(
                        start_x + col as f32 * (BRICK_SIZE.x + 10.0),
                        start_y - row as f32 * (BRICK_SIZE.y + 10.0),
                        0.0,
                    ),
                    texture,
                    ..default()
                },
                Brick { hp },
            ));
        }
    }
}

pub fn spawn_start_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "BRICK BREAKER",
                    TextStyle {
                        font_size: 64.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                StartUI,
            ));

            parent.spawn((
                TextBundle::from_section(
                    "PRESS SPACE TO START",
                    TextStyle {
                        font_size: 36.0,
                        color: Color::YELLOW,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::top(Val::Px(30.0)),
                    ..default()
                }),
                StartUI,
            ));
        });
}

pub fn spawn_game_over_ui(mut commands: Commands, score: Res<Score>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "GAME OVER",
                    TextStyle {
                        font_size: 64.0,
                        color: Color::RED,
                        ..default()
                    },
                ),
                GameOverUI,
            ));

            parent.spawn((
                TextBundle::from_section(
                    format!("Final Score: {}", score.value),
                    TextStyle {
                        font_size: 48.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::top(Val::Px(30.0)),
                    ..default()
                }),
                GameOverUI,
            ));

            parent.spawn((
                TextBundle::from_section(
                    "PRESS R TO RESTART\nPRESS ESC TO EXIT",
                    TextStyle {
                        font_size: 32.0,
                        color: Color::YELLOW,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::top(Val::Px(40.0)),
                    ..default()
                }),
                GameOverUI,
            ));
        });
}

// ============================================================================
// MOVEMENT SYSTEMS
// ============================================================================

pub fn paddle_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut paddle_query: Query<&mut Transform, With<Paddle>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = paddle_query.get_single_mut() {
        let mut direction = 0.0;

        if keyboard_input.pressed(KeyCode::Left) {
            direction -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction += 1.0;
        }

        let movement = direction * PADDLE_SPEED * time.delta_seconds();
        transform.translation.x += movement;
    }
}

pub fn ball_movement(
    mut ball_query: Query<(&mut Transform, &Ball)>,
    time: Res<Time>,
) {
    for (mut transform, ball) in ball_query.iter_mut() {
        let movement = ball.direction * BALL_SPEED * time.delta_seconds();
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
    }
}

pub fn power_up_fall(
    mut query: Query<&mut Transform, With<PowerUp>>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        transform.translation.y -= POWER_UP_SPEED * time.delta_seconds();
    }
}

// ============================================================================
// CONFINEMENT SYSTEMS
// ============================================================================

pub fn confine_paddle(
    mut paddle_query: Query<&mut Transform, With<Paddle>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut paddle_transform) = paddle_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_paddle_width = PADDLE_SIZE.x / 2.0;

        let x_min = -window.width() / 2.0 + half_paddle_width;
        let x_max = window.width() / 2.0 - half_paddle_width;

        let mut translation = paddle_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        paddle_transform.translation = translation;
    }
}

pub fn confine_ball(
    mut ball_query: Query<(Entity, &mut Transform), With<Ball>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
) {
    let window = window_query.get_single().unwrap();

    let half_ball_size = BALL_SIZE / 2.0;
    let x_min = -window.width() / 2.0 + half_ball_size.x;
    let x_max = window.width() / 2.0 - half_ball_size.x;
    let y_min = -window.height() / 2.0 + half_ball_size.y;
    let y_max = window.height() / 2.0 - half_ball_size.y;

    for (ball_entity, mut transform) in ball_query.iter_mut() {
        let mut translation = transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        if translation.y < y_min {
            commands.entity(ball_entity).despawn();
            continue;
        } else if translation.y > y_max {
            translation.y = y_max;
        }
        transform.translation = translation;
    }
}

pub fn update_ball_direction(
    mut ball_query: Query<(&Transform, &mut Ball)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window_query = window_query.get_single().unwrap();

    let half_ball_size = BALL_SIZE / 2.0;
    let x_min = -window_query.width() / 2.0 + half_ball_size.x;
    let x_max = window_query.width() / 2.0 - half_ball_size.x;
    let y_min = -window_query.height() / 2.0 + half_ball_size.y;
    let y_max = window_query.height() / 2.0 - half_ball_size.y;

    for (transform, mut ball) in ball_query.iter_mut() {
        let translation = transform.translation;

        if translation.x <= x_min || translation.x >= x_max {
            ball.direction.x = -ball.direction.x;
        }
        if translation.y <= y_min || translation.y >= y_max {
            ball.direction.y = -ball.direction.y;
        }
    }
}

// ============================================================================
// COLLISION SYSTEMS
// ============================================================================

pub fn ball_paddle_collision(
    mut ball_query: Query<(&mut Transform, &mut Ball), Without<Paddle>>,
    paddle_query: Query<&Transform, With<Paddle>>,
    audio: Res<Audio>,
    assets_server: Res<AssetServer>,
) {
    let paddle_transform = paddle_query.get_single().unwrap();

    for (mut ball_transform, mut ball) in ball_query.iter_mut() {
        let distance_x = (ball_transform.translation.x - paddle_transform.translation.x).abs();
        let distance_y = (ball_transform.translation.y - paddle_transform.translation.y).abs();

        if distance_x <= (BALL_SIZE.x / 2.0 + PADDLE_SIZE.x / 2.0)
            && distance_y <= (BALL_SIZE.y / 2.0 + PADDLE_SIZE.y / 2.0)
            && ball.direction.y < 0.0
        {
            let sound_effect = assets_server.load("audio/impactPunch_heavy_001.ogg");
            audio.play(sound_effect);
            ball.direction.y = -ball.direction.y;

            let overlap_y = (BALL_SIZE.y / 2.0 + PADDLE_SIZE.y / 2.0) - distance_y;
            ball_transform.translation.y += overlap_y;
        }
    }
}

pub fn ball_brick_collision(
    mut commands: Commands,
    mut ball_query: Query<(&mut Transform, &mut Ball), Without<Brick>>,
    mut brick_query: Query<(Entity, &mut Brick, &Transform), Without<Ball>>,
    mut score: ResMut<Score>,
    audio: Res<Audio>,
    assets_server: Res<AssetServer>,
) {
    for (mut ball_transform, mut ball) in ball_query.iter_mut() {
        for (brick_entity, mut brick, brick_transform) in brick_query.iter_mut() {
            let dx = ball_transform.translation.x - brick_transform.translation.x;
            let dy = ball_transform.translation.y - brick_transform.translation.y;

            let overlap_x = (BALL_SIZE.x / 2.0 + BRICK_SIZE.x / 2.0) - dx.abs();
            let overlap_y = (BALL_SIZE.y / 2.0 + BRICK_SIZE.y / 2.0) - dy.abs();

            if overlap_x > 0.0 && overlap_y > 0.0 && brick.hp > 0 {
                brick.hp -= 1;

                let sound_effect = assets_server.load("audio/impactPunch_medium_004.ogg");
                let sound_break = assets_server.load("audio/laserLarge_003.ogg");

                if overlap_x < overlap_y {
                    // va chạm trái / phải
                    audio.play(sound_effect);
                    ball.direction.x = -ball.direction.x;
                    ball_transform.translation.x += overlap_x * dx.signum();
                } else {
                    // va chạm trên / dưới
                    audio.play(sound_effect);
                    ball.direction.y = -ball.direction.y;
                    ball_transform.translation.y += overlap_y * dy.signum();
                }

                if brick.hp == 0 {
                    audio.play(sound_break);
                    commands.entity(brick_entity).despawn();
                    score.value += 10;

                    if random::<f32>() < POWER_UP_DROP_CHANCE {
                        commands.spawn((
                            SpriteBundle {
                                transform: Transform::from_xyz(
                                    brick_transform.translation.x,
                                    brick_transform.translation.y,
                                    0.0,
                                ),
                                texture: assets_server.load("sprites/star.png"),
                                ..default()
                            },
                            PowerUp {},
                        ));
                    }
                }

                break;
            }
        }
    }
}

pub fn paddle_collect_power_up(
    mut commands: Commands,
    paddle_query: Query<&Transform, With<Paddle>>,
    power_up_query: Query<(Entity, &Transform), With<PowerUp>>,
    asset_server: Res<AssetServer>,
) {
    let paddle_transform = paddle_query.get_single().unwrap();

    for (power_up_entity, power_up_transform) in power_up_query.iter() {
        let distance_x = (power_up_transform.translation.x - paddle_transform.translation.x).abs();
        let distance_y = (power_up_transform.translation.y - paddle_transform.translation.y).abs();

        if distance_x <= (PADDLE_SIZE.x / 2.0) && distance_y <= (PADDLE_SIZE.y / 2.0) {
            commands.entity(power_up_entity).despawn();

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(
                        paddle_transform.translation.x,
                        paddle_transform.translation.y + 40.0,
                        0.0,
                    ),
                    texture: asset_server.load("sprites/ballBlue.png"),
                    ..default()
                },
                Ball {
                    direction: Vec2::new(random::<f32>(), 1.0).normalize(),
                },
            ));
        }
    }
}

// ============================================================================
// GAME STATE MANAGEMENT
// ============================================================================

pub fn start_game(
    keyboard: Res<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
    mut commands: Commands,
    ui_query: Query<Entity, With<StartUI>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for e in ui_query.iter() {
            commands.entity(e).despawn_recursive();
        }
        state.set(GameState::Playing).unwrap();
    }
}

pub fn check_game_over(
    ball_query: Query<(), With<Ball>>,
    score: Res<Score>,
    mut game_over_events: EventWriter<GameOver>,
) {
    if ball_query.is_empty() {
        game_over_events.send(GameOver {
            score: score.value,
        });
    }
}

pub fn handle_game_over(
    mut game_over_events: EventReader<GameOver>,
    mut state: ResMut<State<GameState>>,
) {
    for event in game_over_events.iter() {
        println!("Game Over! Your final score is: {}", event.score);
        state.set(GameState::GameOver).unwrap();
    }
}

pub fn restart_game(
    keyboard: Res<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
    mut commands: Commands,
    mut score: ResMut<Score>,
    ui_query: Query<Entity, With<GameOverUI>>,
) {
    if keyboard.just_pressed(KeyCode::R) {
        // Despawn UI
        for e in ui_query.iter() {
            commands.entity(e).despawn_recursive();
        }
        // Reset score
        score.value = 0;
        // Change state to Playing (this will trigger OnEnter and spawn entities)
        state.set(GameState::Playing).unwrap();
    }
}

// ============================================================================
// CLEANUP SYSTEMS
// ============================================================================

pub fn cleanup_game_entities(
    mut commands: Commands,
    paddle_query: Query<Entity, With<Paddle>>,
    ball_query: Query<Entity, With<Ball>>,
    brick_query: Query<Entity, With<Brick>>,
    power_up_query: Query<Entity, With<PowerUp>>,
) {
    for entity in paddle_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in ball_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in brick_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in power_up_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// ============================================================================
// UTILITY SYSTEMS
// ============================================================================

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
}
