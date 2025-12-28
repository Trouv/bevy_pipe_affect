use bevy::prelude::*;
use bevy_pipe_affect::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Update,
            pure(rainbow_clear_color) // pure() is optional, just forces the system to be read-only
                .pipe(affect),
        )
        .run()
}

/// This system defines the clear color as a pure function of time.
fn rainbow_clear_color(time: Res<Time>) -> ResSet<ClearColor> {
    let color = Color::hsv(time.elapsed_secs() * 20.0, 0.7, 0.7);
    res_set(ClearColor(color))
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use super::*;

    #[test]
    fn test_rainbow_clear() {
        // Create a World with a Resource for our Time
        let mut world = World::new();
        world.insert_resource::<Time>(Time::default());

        // Now that we have a Res<Time>, we can run our system and get an output
        let output = world.run_system_once(rainbow_clear_color).unwrap();

        // Then all we have to do is check that output has the proper value
        assert_eq!(output.value.0, Color::hsva(0.0, 0.7, 0.7, 1.0));
    }
}
