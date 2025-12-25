//! This example implements simple sokoban pushing logic with one extra wrinkle: block weight.
//!
//! Aside from basic effect usage, this demonstrates...
//! - recursive EffectOut composition
//! - observer workflows
//!
//! Some code examples from the book are pulled from here.
use bevy::prelude::*;
use bevy_pipe_affect::prelude::*;

use crate::animate::animate_position;
use crate::player::{player_input, spawn_player};
use crate::push::spawn_push_observer;
use crate::setup::{spawn_blocks, spawn_camera};

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
