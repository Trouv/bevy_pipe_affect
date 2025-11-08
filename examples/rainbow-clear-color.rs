use bevy::prelude::*;
use bevy_pipe_affect::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Update,
            pure(rainbow_clear_color) // pure() is optional, just forces the system to be read-only
                .pipe(affect),
        )
        .run();
}

/// This system defines the clear color as a pure function of time.
fn rainbow_clear_color(time: Res<Time>) -> impl Effect {
    let color = Color::hsv(time.elapsed_secs() * 20.0, 0.7, 0.7);
    res_set(ClearColor(color))
}
