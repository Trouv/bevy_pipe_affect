use bevy::prelude::*;
use bevy_pipe_affect::prelude::*;

use crate::player::Player;
use crate::push::{push, Position, Weight};

pub const BLOCK_SIZE: Vec2 = Vec2 { x: 32.0, y: 32.0 };

pub fn spawn_player() -> impl Effect {
    command_spawn((
        Player,
        Weight(0),
        Position(IVec2::ZERO),
        Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), BLOCK_SIZE),
    ))
}

pub fn spawn_blocks() -> Vec<impl Effect> {
    (1..10)
        .map(|block_num| {
            let color_value = (10 - block_num) as f32 / 10.0;

            command_spawn((
                Weight(block_num as u32),
                Position(IVec2::splat(block_num)),
                Sprite::from_color(
                    Color::srgb(color_value, color_value, color_value),
                    BLOCK_SIZE,
                ),
            ))
        })
        .collect()
}

pub fn spawn_camera() -> impl Effect {
    command_spawn(Camera2d)
}

pub fn spawn_push_observer() -> impl Effect {
    command_spawn(Observer::new(push.pipe(affect)))
}
