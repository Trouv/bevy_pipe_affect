use bevy::prelude::*;
use bevy_pipe_affect::prelude::*;

use crate::push::{Position, PushEntity, Weight};
use crate::setup::BLOCK_SIZE;

/// A marker component for the player entity..
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Component)]
pub struct Player;

/// Defines how `PushEntity` events are triggered based off keyboard input.
pub fn player_input(
    input: Res<ButtonInput<KeyCode>>,
    player: Single<Entity, With<Player>>,
) -> Option<CommandTrigger<PushEntity>> {
    if input.just_pressed(KeyCode::ArrowRight) {
        Some(command_trigger(PushEntity {
            direction: IVec2::X,
            entity: *player,
        }))
    } else if input.just_pressed(KeyCode::ArrowUp) {
        Some(command_trigger(PushEntity {
            direction: IVec2::Y,
            entity: *player,
        }))
    } else if input.just_pressed(KeyCode::ArrowLeft) {
        Some(command_trigger(PushEntity {
            direction: -IVec2::X,
            entity: *player,
        }))
    } else if input.just_pressed(KeyCode::ArrowDown) {
        Some(command_trigger(PushEntity {
            direction: -IVec2::Y,
            entity: *player,
        }))
    } else {
        None
    }
}

/// Spawns the player entity (a block with a Player component).
pub fn spawn_player() -> impl Effect {
    command_spawn((
        Player,
        Weight(0),
        Position(IVec2::ZERO),
        Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), BLOCK_SIZE),
    ))
}
