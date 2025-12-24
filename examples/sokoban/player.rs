use bevy::prelude::*;
use bevy_pipe_affect::prelude::*;

use crate::push::PushEntity;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Component)]
pub struct Player;

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
