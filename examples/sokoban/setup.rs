use bevy::prelude::*;
use bevy_pipe_affect::prelude::*;

use crate::push::{Position, Weight};

/// The size of the grid cells/blocks.
pub const BLOCK_SIZE: Vec2 = Vec2 { x: 32.0, y: 32.0 };

/// Spawns 10 blocks of varying color/weight/position.
pub fn spawn_blocks() -> Vec<CommandSpawn<(Weight, Position, Sprite)>> {
    (1..=10)
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

/// Spawns a simple camera entity.
pub fn spawn_camera() -> CommandSpawn<Camera2d> {
    command_spawn(Camera2d)
}
