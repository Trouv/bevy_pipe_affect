//! Basic example of pure observer usage.
//!
//! Snippets of this are used in the *Spawn and Trigger an Observer* how-to-guide in the book.

use bevy::prelude::*;
use bevy_pipe_affect::prelude::*;

use crate::inflatable::{Inflatable, spawn_observer, trigger_observer};

mod inflatable;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (setup.pipe(affect), spawn_observer.pipe(affect)))
        .add_systems(Update, trigger_observer.pipe(affect))
        .run();
}

pub fn setup() -> (
    CommandSpawn<Camera2d>,
    AssetServerLoadAnd<'static, Image, CommandSpawn<(Sprite, Inflatable)>>,
) {
    (
        command_spawn(Camera2d::default()),
        asset_server_load_and("player.png", |sprite| {
            command_spawn((Sprite::from_image(sprite), Inflatable))
        }),
    )
}
