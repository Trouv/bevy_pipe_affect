use bevy::prelude::*;
use bevy_pipe_affect::prelude::*;

use crate::animate::animate_position;
use crate::player::player_input;
use crate::setup::{spawn_blocks, spawn_camera, spawn_player, spawn_push_observer};

mod push;

mod player;

mod setup;

mod animate;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (
                spawn_camera.pipe(affect),
                spawn_player.pipe(affect),
                spawn_blocks.pipe(affect),
                spawn_push_observer.pipe(affect),
            ),
        )
        .add_systems(
            Update,
            (animate_position.pipe(affect), player_input.pipe(affect)),
        )
        .run();
}
