use bevy::prelude::*;

#[derive(Component)]
pub struct Ball{
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Brick{
    pub hp: u8,
}

#[derive(Component)]
pub struct Paddle{}

#[derive(Component)]
pub struct StartUI;

#[derive(Component)]
pub struct GameOverUI;
